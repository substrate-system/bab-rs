import { FunctionComponent, render } from 'preact'
import { html } from 'htm/preact'
import Debug from '@substrate-system/debug'
import { humanBytes } from '@substrate-system/human-bytes'
import {
    buildVerificationMetadata,
    verifyChunk,
} from './bab-verify.js'
import init, { william3_width } from '../pkg/bab.js'
import { State } from './state.js'
import { bytesToHex, base64ToBytes } from './util.js'
import '@substrate-system/css-normalize'

const debug = Debug(import.meta.env.DEV || import.meta.env.MODE === 'staging')
const NBSP = '\u00A0'

// Initialize the WASM module
await init();

debug('WILLIAM3 digest width:', william3_width(), 'bytes');

const state = State()

// Build trusted root from the original image
function buildTrustedRoot() {
    const bytes = base64ToBytes(state.imageData.value);
    const metadata = buildVerificationMetadata(bytes);

    state.trustedRootHash.value = bytesToHex(metadata.rootDigest);
    state.verificationMetadata.value = metadata.chunks;
    state.verifiedChunks.value = new Map();

    debug('Built trusted root:', state.trustedRootHash.value);
    debug('Number of chunks:', metadata.chunks.length);
}

// Verify a chunk
function verifyChunkAtIndex(index: number) {
    if (!state.trustedRootHash.value || !state.verificationMetadata.value.length) {
        return;
    }

    const bytes = base64ToBytes(state.imageData.value);
    const metadata = state.verificationMetadata.value[index];
    const trustedRoot = new Uint8Array(32);

    for (let i = 0; i < 32; i++) {
        trustedRoot[i] = parseInt(state.trustedRootHash.value.substr(i * 2, 2), 16);
    }

    const isValid = verifyChunk(
        metadata.data,
        metadata,
        state.verificationMetadata.value.length,
        trustedRoot
    );

    const newMap = new Map(state.verifiedChunks.value);
    newMap.set(index, isValid);
    state.verifiedChunks.value = newMap;
}

// Stream and verify all chunks
async function streamAndVerify() {
    if (!state.trustedRootHash.value) {
        alert('Please build the trusted root first!');
        return;
    }

    state.isStreaming.value = true;
    state.verifiedChunks.value = new Map();

    for (let i = 0; i < state.verificationMetadata.value.length; i++) {
        await new Promise(resolve => setTimeout(resolve, 100)); // Simulate network delay
        verifyChunkAtIndex(i);
    }

    state.isStreaming.value = false;
}

// Components
const ImageDisplay: FunctionComponent = () => {
    return html`
        <div class="image-display">
            <h3>Llama Image</h3>
            <img src=${state.imageData.value} alt="Llama" />
        </div>
    `;
};

const TrustedRootSection: FunctionComponent = () => {
    const numChunks = state.verificationMetadata.value.length;

    return html`
        <div class="method">
            <h2>Step 1: Build Trusted Root</h2>
            <p class="description">
                Hash the complete image to create the trusted root digest.
                This simulates receiving the root hash through a secure channel.
            </p>
            <button onClick=${buildTrustedRoot} disabled=${state.isStreaming.value}>
                Build Trusted Root
            </button>

            ${state.trustedRootHash.value && html`
                <div class="output success">
                    <strong>Trusted Root Hash:</strong><br/>
                    <code class="hash">${state.trustedRootHash.value}</code><br/>
                    <strong>Total Chunks:</strong> ${numChunks}<br/>
                    <strong>Chunk Size:</strong> 1024 bytes
                </div>
            `}
        </div>
    `;
};

const ChunkItem: FunctionComponent<{ index: number }> = ({ index }) => {
    const metadata = state.verificationMetadata.value[index];
    const verificationStatus = state.verifiedChunks.value.get(index);

    return html`
        <div class="chunk-item">
            <div>
                <strong>Chunk ${index}</strong>
                <span>
                    ${metadata.data.length} bytes
                </span>
            </div>

            ${verificationStatus !== undefined && html`
                <div class="chunk-result">
                    ${verificationStatus
                        ? html`<div class="success">✓ Verified</div>`
                        : html`<div class="error">✗ Verification Failed</div>`
                    }
                </div>
            `}
        </div>
    `;
};

const StreamingSection:FunctionComponent = () => {
    return html`
        <div class="method">
            <h2>Step 2: Stream and Verify Chunks</h2>
            <p class="description">
                Simulate receiving chunks over a network and verifying each one
                as it arrives using the trusted root.
            </p>

            ${state.trustedRootHash.value && html`
                <button
                    onClick=${streamAndVerify}
                    disabled=${state.isStreaming.value}
                >
                    ${state.isStreaming.value ? 'Streaming...' : 'Start Streaming'}
                </button>

                <div class="chunks-list">
                    ${state.verificationMetadata.value.map((_, i) =>
                        html`<${ChunkItem} key=${i} index=${i} />`
                    )}
                </div>
            `}

            ${!state.trustedRootHash.value && html`
                <p class="info">Build the trusted root first to enable streaming.</p>
            `}
        </div>
    `;
};

const BabDemo: FunctionComponent = () => {
    return html`
        <h1>Bab WILLIAM3 Hash Function</h1>

        <p>
            Use bab for incremental (streaming) verification.
        </p>

        <h2>The Process</h2>

        <p>
            First we get the hash of the image,
            then we create a${NBSP}
            <a href="https://developer.mozilla.org/en-US/docs/Web/API/Streams_API">
                web stream
            </a>.
            The stream contains ordered chunks of the file, and each chunk has
            some metadata prefixed to it.
        </p>

        <p>
            The receiver of the file can then verify that each chunk is correct
            as they stream in. You don't have to wait for the full file and
            then construct a hash. This way you can potentially save bandwidth
            since you are able to abort a transfer as soon as it is visibly
            invalid. You don't have to wait for the entire blob to arrive.
        </p>

        <p>
            The stream originates right here in your browser, in this page,
            but the stream has the same API as a blob received from over the
            network, so it's good for a demo.
        </p>

        <${ImageDisplay} />
        <${DataTextarea} />
        <${TrustedRootSection} />
        <${StreamingSection} />
    `;
};

render(html`<${BabDemo} />`, document.getElementById('root')!);

function DataTextarea () {
    return html`
        <div class="data-section">
            <h3>Base64 Image Data</h3>
            <p class="description">
                This is the base64-encoded image data.
                You can modify it to test verification failure.
            </p>
            <textarea
                value=${state.imageData.value}
                onInput=${(e: Event) => {
                    state.imageData.value = (e.target as HTMLTextAreaElement).value;
                    // Reset verification when data changes
                    state.trustedRootHash.value = '';
                    state.verificationMetadata.value = [];
                    state.verifiedChunks.value = new Map();
                }}
            ></textarea>
            <p class="info">
                Data size: ${humanBytes(base64ToBytes(state.imageData.value).length)}
            </p>
        </div>
    `;
};
