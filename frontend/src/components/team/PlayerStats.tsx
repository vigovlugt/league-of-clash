import IChampionStats from "../../models/IChampionStats";
import ChampionStats from "./ChampionStats";

interface IProps {
    championStats: IChampionStats[];
    summonerName: string;
}

const PlayerStats: React.FC<IProps> = ({ summonerName, championStats }) => (
    <div>
        <h3 className="text-3xl font-header">{summonerName}</h3>

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
                    {championStats.slice(0, 5).map((c) => (
                        <ChampionStats key={c.champion_id} championStats={c} />
                    ))}
                </tbody>
            </table>
        </div>
    </div>
);

export default PlayerStats;
