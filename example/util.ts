// Convert Uint8Array to hex string
export function bytesToHex(bytes:Uint8Array):string {
    return Array.from(bytes)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('')
}

// Convert base64 to Uint8Array
export function base64ToBytes(base64:string):Uint8Array {
    // Remove data URI prefix if present
    const b64Data = base64.includes(',') ? base64.split(',')[1] : base64;
    const binaryString = atob(b64Data);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
}

