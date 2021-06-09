import { useState } from "react";
import IPlayerStats, { getWinrate } from "../../models/IPlayerStats";
import { getRankClass } from "../../utils/rank";
import ChampionStats from "../champion/ChampionStats";

interface IProps {
    playerStats: IPlayerStats;
}

const PlayerStats: React.FC<IProps> = ({ playerStats }) => {
    const [showMore, setShowMore] = useState(false);

    const showRank = !["MASTER", "GRANDMASTER", "CHALLENGER"].includes(
        playerStats.tier
    );

    return (
        <div>
            <div className="flex justify-between">
                <a
                    href={`https://u.gg/lol/profile/euw1/${playerStats.summoner_name}/overview`}
                    target="_blank"
                >
                    <h3 className="text-3xl font-header">
                        {playerStats.summoner_name}
                        {" - "}
                        <span className={getRankClass(playerStats.tier)}>{`${
                            playerStats.tier
                        } ${showRank ? playerStats.rank : ""}`}</span>
                    </h3>
                </a>
                <h3 className="text-3xl font-header">
                    {playerStats.games} GAMES
                </h3>
            </div>
            <div className="flex justify-between">
                <h3 className="text-3xl font-header">
                    {getWinrate(playerStats)} WR
                </h3>
                <h3 className="text-3xl font-header">
                    {playerStats.champion_stats.length} - CHAMPIONS
                </h3>
            </div>

            <div className="rounded overflow-hidden mt-1">
                <table className="min-w-full divide-y divide-gray-700">
                    <thead className="bg-light-dark">
                        <tr>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">
                                Champion
                            </th>
                            <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                                Winrate
                            </th>
                            <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                                Games
                            </th>
                            <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider">
                                KDA
                            </th>
                            <th className="px-6 py-3 text-right text-xs font-medium text-gray-300 uppercase tracking-wider whitespace-nowrap">
                                Carry
                            </th>
                            <th className="px-6 py-3 text-right text-xs font-medium text-primary uppercase tracking-wider">
                                Score
                            </th>
                        </tr>
                    </thead>
                    <tbody className="bg-gray-800 divide-y divide-gray-700">
                        {playerStats.champion_stats
                            .slice(0, showMore ? undefined : 5)
                            .map((c) => (
                                <ChampionStats
                                    key={c.champion_id}
                                    championStats={c}
                                />
                            ))}
                    </tbody>
                </table>
            </div>
            <div className="flex">
                <button
                    className="bg-light-dark px-3 py-2 mt-2 font-bold border-b-4 border-gray-900 active:border-b-0 active:mt-3 focus:outline-none"
                    onClick={() => setShowMore(!showMore)}
                >
                    {showMore ? "Show less" : "Show all"}
                </button>
            </div>
        </div>
    );
};

export default PlayerStats;
