// Display-label normalization for parsed resolution/codec values.
//
// parse() returns raw detection values (e.g. resolution "4k", codec "hevc").
// These helpers map them to the canonical display labels consumers expect
// (e.g. "4K", "HEVC"). Kept separate from parsing so callers can normalize
// on demand without a second parse.

const RESOLUTION_MAP = {
    "4k": "4K",
    "2160p": "4K",
    "1440p": "1440p",
    "1080p": "1080p",
    "720p": "720p",
    "576p": "576p",
    "480p": "480p",
    "360p": "360p",
    "240p": "240p",
};

// Reachable video-codec values from parse().codec plus common aliases.
const CODEC_MAP = {
    h264: "x264",
    x264: "x264",
    avc: "AVC",
    h265: "x265",
    x265: "x265",
    hevc: "HEVC",
    x266: "x266",
    vvc: "VVC",
    av1: "AV1",
    divx: "DivX",
    dvix: "DivX",
    xvid: "XviD",
    mpeg2: "MPEG2",
};

/**
 * Normalize a parsed resolution value to its display label.
 * Returns null for falsy input, or the original value if unmapped.
 */
function normalizeResolution(resolution) {
    if (!resolution) return null;
    const key = String(resolution).toLowerCase().replace(/[\s._-]/g, "");
    return RESOLUTION_MAP[key] || resolution;
}

/**
 * Normalize a parsed codec value to its display label.
 * Returns null for falsy input, or the original value if unmapped.
 */
function normalizeCodec(codec) {
    if (!codec) return null;
    const key = String(codec).toLowerCase().replace(/[\s._-]/g, "");
    return CODEC_MAP[key] || codec;
}

module.exports = { normalizeResolution, normalizeCodec, RESOLUTION_MAP, CODEC_MAP };
