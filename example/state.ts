import llamaBase64 from "./llama.jpg.base64";
import { signal } from "@preact/signals";
import { type ChunkMetadata } from './bab-verify.js'

export function State () {
    const imageData = signal(llamaBase64);
    const trustedRootHash = signal<string>('');
    const verificationMetadata = signal<ChunkMetadata[]>([]);
    const verifiedChunks = signal<Map<number, boolean>>(new Map());
    const isStreaming = signal(false);

    return {
        imageData,
        trustedRootHash,
        verificationMetadata,
        verifiedChunks,
        isStreaming
    }
}
