import React from "react";
import DraftContext from "../../context/DraftContext";
import IChampionStats, {
    getCarryScore,
    getKda,
    getWinrate,
} from "../../models/IChampionStats";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    championStats: IChampionStats;
}

const ChampionStats: React.FC<IProps> = ({ championStats }) => {
    const { championData } = React.useContext(DraftContext);
    const champion = championData[championStats.champion_id.toString()];

    return (
        <tr>
            <td className="px-6 py-4 whitespace-nowrap">
                <div className="flex items-center">
                    <div className="relative w-12 h-12 min-w-12 min-h-12 overflow-hidden  mr-4">
                        <img
                            src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                            className="w-12 h-12 min-w-12 min-h-12 absolute inset-0"
                            style={{ transform: "scale(1.1,1.1)" }}
                        ></img>
                    </div>
                    <h4 className="text-xl font-header">{champion.name}</h4>
                </div>
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-right">
                {getWinrate(championStats)}
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-right">
                {championStats.games}
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-right">
                {getKda(championStats)}
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-right">
                {getCarryScore(championStats)}
            </td>
            <td className="px-6 py-4 whitespace-nowrap text-right text-primary">
                {(championStats.score * 100).toFixed(2)}
            </td>
        </tr>
    );
};

export default ChampionStats;
