// Filename-tuned episode extraction.
//
// parse() targets release *titles*; batch torrents expose individual *filenames*
// that use looser conventions ("[Group] Show - 01 [720p].mkv", "第01話", "01화").
// parseFilename() strips technical noise then extracts the episode number, and is
// intended as the file-list counterpart to parse().episode.
//
// Ported from the Nyaa scraper (streams/batch.js parseFileEpisodeNumber).

function sanitizeFilename(filename) {
    return filename
        .replace(/\.(mkv|mp4|avi|wmv|flv|webm|m4v|ts|mov|srt|ass|ssa|vtt|sub|idx)$/i, "")
        .replace(/^\[.*?\]/g, "")
        .replace(/\b(?:\d{3,4}x\d{3,4})\b/gi, "")
        .replace(/\b(?:2160|1080|810|720|540|480|360)[pix]*\b/gi, "")
        .replace(/\b(?:x|h)26[45]\b/gi, "")
        .replace(/\b(?:HEVC|AVC|FHD|HD|SD|10-?bits?|8-?bits?|12-?bits?|Hi10P|Hi444P)\b/gi, "")
        .replace(/\b(?:BD|BDRip|Blu-?ray|WEB-?DL|WEB-?Rip|DVD|DVDRip|TVRip|HDTV|CAM)\b/gi, "")
        .replace(/\b(?:FLAC|AAC|AC3|DTS|DTS-HD|TrueHD|Vorbis|Opus|MP3|PCM)\b/gi, "")
        .replace(/\b(?:Uncensored|Censored|Decensored|Uncen|Dual-?Audio|Multi-?Subs|RAW|Hentai)\b/gi, "")
        .replace(/\b(?:5\.1|2\.0|7\.1|2\.1)\b/g, "")
        .replace(/\[[a-fA-F0-9]{8}\]/g, "")
        .replace(/\b(?:NC)?(?:OP|ED|Opening|Ending)\s*\d*\b/gi, " ")
        .replace(/\b(?:v\d)\b/gi, "")
        .replace(/\b(?:19|20)\d{2}\b/g, "");
}

/**
 * parseFilename
 * Extract an episode number from a batch-torrent filename / path string.
 * Returns null if no episode number is found.
 */
function parseFilename(filename) {
    if (!filename) return null;

    // Use only the basename (last path segment) to avoid false positives in dirs
    const base = filename.split(/[\\/]/).pop() || filename;

    let clean = sanitizeFilename(base);

    // Strip season markers so they don't bleed into episode number capture
    clean = clean.replace(/(?:第|시즌\s*)?0*\d+\s*(?:季|期|기)/ig, "");

    // 1. Explicit keyword prefixes (highest specificity)
    const explicitMatch = clean.match(
        /(?:ep(?:isode)?\.?\s*|\be\s*|ova\s*|oad\s*|special\s*|round\s*|act\s*|chapter\s*|part\s*|vol(?:ume)?\.?\s*|第\s*|#\s*|s(\d+)\s*e|season\s*(\d+)\s*ep(?:isode)?\s*)0*(\d+)(?:\s*(?:巻|話|话|集|화|회|편|v\d+))?(?:\D|$)/i
    );
    if (explicitMatch) {
        const fileSeason = explicitMatch[1] || explicitMatch[2];
        if (fileSeason !== undefined) {
            const parsedSeason = parseInt(fileSeason, 10);
            if (parsedSeason !== 1) return null;
        }
        const n = parseInt(explicitMatch[3], 10);
        if (n >= 1900 && n <= 2099) return null;
        return n;
    }

    // 2. Dash-separated episode: " - 01"
    const dashMatch = clean.match(/(?:^|\s)-\s+0*(\d+)(?:\D|$)/i);
    if (dashMatch) {
        const n = parseInt(dashMatch[1], 10);
        if (n >= 1900 && n <= 2099) return null;
        return n;
    }

    // 3. Bracket-wrapped: "[01]" / "(01)"
    const bracketMatch = clean.match(/\[0*(\d+)\]|\(0*(\d+)\)/i);
    if (bracketMatch) {
        const n = parseInt(bracketMatch[1] || bracketMatch[2], 10);
        if (n >= 1900 && n <= 2099) return null;
        return n;
    }

    // 4. Last standalone number token (right-to-left scan)
    clean = clean.replace(/[\[\]\(\)\{\}_\-\+~,#]/g, " ").trim();
    const tokens = clean.split(/\s+/);
    for (let i = tokens.length - 1; i >= 0; i--) {
        const m = tokens[i].match(/^e?0*(\d+)(?:v\d+)?$/i);
        if (m) {
            const n = parseInt(m[1], 10);
            if (n >= 1900 && n <= 2099) continue;
            return n;
        }
    }

    return null;
}

module.exports = { parseFilename, sanitizeFilename };
