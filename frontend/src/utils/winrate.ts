export function getWinrate(wins: number, games: number) {
    return ((wins / games) * 100).toFixed(2);
}

export function winrateClass(wins: number, games: number) {
    if (games === 0) {
        return "text-winrate-okay";
    }

    const winrate = wins / games;

    if (winrate < 0.45) {
        return "text-winrate-shiggo";
    } else if (winrate < 0.485) {
        return "text-winrate-meh";
    } else if (winrate < 0.515) {
        return "text-winrate-okay";
    } else if (winrate < 0.53) {
        return "text-winrate-good";
    } else if (winrate < 0.55) {
        return "text-winrate-great";
    }

    return "text-winrate-volxd";
}

export function winrateBorderClass(wins: number, games: number) {
    if (games === 0) {
        return "border-winrate-okay";
    }

    const winrate = wins / games;

    if (winrate < 0.45) {
        return "border-winrate-shiggo";
    } else if (winrate < 0.485) {
        return "border-winrate-meh";
    } else if (winrate < 0.515) {
        return "border-winrate-okay";
    } else if (winrate < 0.53) {
        return "border-winrate-good";
    } else if (winrate < 0.55) {
        return "border-winrate-great";
    }

    return "border-winrate-volxd";
}
