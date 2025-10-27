import init, {
    william3_hash,
    William3HasherWasm,
    william3_width
} from '../pkg/bab.js'

// Initialize the WASM module
await init();

const byId = document.getElementById.bind(document)

console.log('WILLIAM3 digest width:', william3_width(), 'bytes');

// Make functions available globally
// @ts-expect-error
const hashBatch = window.hashBatch = function() {
    const input = (document.getElementById('input') as HTMLInputElement).value;
    const encoder = new TextEncoder();
    const data = encoder.encode(input);

    const hash = william3_hash(data);

    byId('output')!.innerHTML = `
        <strong>Input:</strong> ${input}<br>
        <strong>Hash:</strong> ${hash}
    `;
};

// @ts-expect-error
const hashIncremental = window.hashIncremental = function() {
    const input1 = (byId('input1') as HTMLInputElement).value;
    const input2 = (byId('input2') as HTMLInputElement).value;

    const encoder = new TextEncoder();
    const hasher = new William3HasherWasm();

    hasher.write(encoder.encode(input1));
    hasher.write(encoder.encode(input2));

    const hash = hasher.finish_hex();

    byId('output2')!.innerHTML = `
        <strong>Part 1:</strong> ${input1}<br>
        <strong>Part 2:</strong> ${input2}<br>
        <strong>Combined input:</strong> ${input1}${input2}<br>
        <strong>Hash:</strong> ${hash}
    `;
};

// Hash the default value on load
hashBatch();
hashIncremental();
