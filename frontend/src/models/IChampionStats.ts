export default interface IChampionStats {
    champion_id: number;
    kills: number;
    deaths: number;
    assists: number;
    kill_participation: number;
    ps_hard_carry: number;
    ps_team_play: number;
    wins: number;
    games: number;
    score: number;
}

export function getKda(stats: IChampionStats) {
    return ((stats.kills + stats.assists) / stats.deaths).toFixed(2);
}

export function getCarryScore(stats: IChampionStats) {
    return ((stats.ps_hard_carry + stats.ps_team_play) / 2).toFixed(2);
}

export function getWinrate(stats: IChampionStats) {
    return ((stats.wins / stats.games) * 100).toFixed(2);
}
