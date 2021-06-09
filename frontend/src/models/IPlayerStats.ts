import IChampionStats from "./IChampionStats";

export default interface IPlayerStats {
    summoner_name: string;

    wins: number;
    games: number;

    rank: string;
    tier: string;

    champion_stats: IChampionStats[];
}

export function getWinrate(stats: IPlayerStats) {
    return ((stats.wins / stats.games) * 100).toFixed(2);
}
