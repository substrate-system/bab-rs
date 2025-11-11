import { William3HasherWasm, william3_hash } from '../pkg/bab.js'

const WIDTH = 32; // WILLIAM3 digest width in bytes
const CHUNK_SIZE = 1024; // Bab default chunk size

export interface TreeNode {
    label: Uint8Array;
    startChunk: number;
    endChunk: number;
    left?: TreeNode;
    right?: TreeNode;
}

export interface ChunkMetadata {
    chunkIndex: number;
    data: Uint8Array;
    label: Uint8Array;
    siblings: Uint8Array[];
    directions: boolean[]; // true = left, false = right
    mergedLengths: number[];
}

export interface VerificationMetadata {
    rootDigest: Uint8Array;
    chunks: ChunkMetadata[];
}

// Convert hex string to Uint8Array
function hexToBytes(hex: string): Uint8Array {
    const bytes = new Uint8Array(hex.length / 2);
    for (let i = 0; i < hex.length; i += 2) {
        bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
    }
    return bytes;
}

// Convert Uint8Array to hex string
function bytesToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
}

// Hash a chunk using WILLIAM3
function hashChunk(chunk: Uint8Array, isRoot: boolean): Uint8Array {
    const hasher = new William3HasherWasm();
    hasher.write(chunk);
    const hex = hasher.finish_hex();
    return hexToBytes(hex);
}

// Hash an inner node using WILLIAM3
function hashInner(
    leftLabel: Uint8Array,
    rightLabel: Uint8Array,
    lengthOfSubtree: number,
    isRoot: boolean
): Uint8Array {
    const hasher = new William3HasherWasm();
    hasher.write(leftLabel);
    hasher.write(rightLabel);
    const hex = hasher.finish_hex();
    return hexToBytes(hex);
}

// Build Merkle tree from data
function buildMerkleTree(data: Uint8Array): TreeNode {
    // Split data into chunks
    const numChunks = Math.ceil(data.length / CHUNK_SIZE);
    const leaves: TreeNode[] = [];

    for (let i = 0; i < numChunks; i++) {
        const start = i * CHUNK_SIZE;
        const end = Math.min(start + CHUNK_SIZE, data.length);
        const chunkData = data.slice(start, end);
        const label = hashChunk(chunkData, numChunks === 1);

        leaves.push({
            label,
            startChunk: i,
            endChunk: i
        });
    }

    if (leaves.length === 0) {
        // Empty data
        const label = hashChunk(new Uint8Array(0), true);
        return { label, startChunk: 0, endChunk: 0 };
    }

    if (leaves.length === 1) {
        return leaves[0];
    }

    // Build tree bottom-up
    return buildTreeLevel(leaves, data.length);
}

function buildTreeLevel(nodes: TreeNode[], totalLength: number): TreeNode {
    if (nodes.length === 1) {
        return nodes[0];
    }

    const parents: TreeNode[] = [];

    for (let i = 0; i < nodes.length; i += 2) {
        if (i + 1 < nodes.length) {
            // Two children
            const left = nodes[i];
            const right = nodes[i + 1];
            const subtreeLength = (right.endChunk - left.startChunk + 1) * CHUNK_SIZE;
            const isRoot = parents.length === 0 && i + 2 >= nodes.length;

            const label = hashInner(left.label, right.label, subtreeLength, isRoot);

            parents.push({
                label,
                startChunk: left.startChunk,
                endChunk: right.endChunk,
                left,
                right
            });
        } else {
            // Odd node, carry to next level
            parents.push(nodes[i]);
        }
    }

    return buildTreeLevel(parents, totalLength);
}

// Extract verification path for a specific chunk
function extractVerificationPath(
    root: TreeNode,
    chunkIndex: number
): { siblings: Uint8Array[], directions: boolean[], mergedLengths: number[] } {
    const siblings: Uint8Array[] = [];
    const directions: boolean[] = [];
    const mergedLengths: number[] = [];

    function traverse(node: TreeNode): boolean {
        if (node.startChunk === node.endChunk && node.startChunk === chunkIndex) {
            return true;
        }

        if (!node.left || !node.right) {
            return false;
        }

        if (traverse(node.left)) {
            siblings.push(node.right.label);
            directions.push(true); // We went left
            mergedLengths.push((node.endChunk - node.startChunk + 1) * CHUNK_SIZE);
            return true;
        }

        if (traverse(node.right)) {
            siblings.push(node.left.label);
            directions.push(false); // We went right
            mergedLengths.push((node.endChunk - node.startChunk + 1) * CHUNK_SIZE);
            return true;
        }

        return false;
    }

    traverse(root);
    return { siblings, directions, mergedLengths };
}

// Build verification metadata for all chunks
export function buildVerificationMetadata(data: Uint8Array): VerificationMetadata {
    const tree = buildMerkleTree(data);
    const numChunks = Math.ceil(data.length / CHUNK_SIZE);
    const chunks: ChunkMetadata[] = [];

    for (let i = 0; i < numChunks; i++) {
        const start = i * CHUNK_SIZE;
        const end = Math.min(start + CHUNK_SIZE, data.length);
        const chunkData = data.slice(start, end);
        const label = hashChunk(chunkData, numChunks === 1);
        const { siblings, directions, mergedLengths } = extractVerificationPath(tree, i);

        chunks.push({
            chunkIndex: i,
            data: chunkData,
            label,
            siblings,
            directions,
            mergedLengths
        });
    }

    return {
        rootDigest: tree.label,
        chunks
    };
}

// Verify a single chunk
export function verifyChunk(
    chunkData: Uint8Array,
    metadata: ChunkMetadata,
    numChunks: number,
    trustedRoot: Uint8Array
): boolean {
    // Compute chunk label
    let currentLabel = hashChunk(chunkData, numChunks === 1);

    if (numChunks === 1) {
        // Single chunk, compare directly
        return bytesToHex(currentLabel) === bytesToHex(trustedRoot);
    }

    // Traverse up the tree
    for (let i = metadata.siblings.length - 1; i >= 0; i--) {
        const sibling = metadata.siblings[i];
        const wentLeft = metadata.directions[i];
        const mergedLength = metadata.mergedLengths[i];
        const isRoot = i === 0;

        if (wentLeft) {
            currentLabel = hashInner(currentLabel, sibling, mergedLength, isRoot);
        } else {
            currentLabel = hashInner(sibling, currentLabel, mergedLength, isRoot);
        }
    }

    return bytesToHex(currentLabel) === bytesToHex(trustedRoot);
}
