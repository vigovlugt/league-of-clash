import React from "react";
import { useDrag } from "react-dnd";
import { CHAMPION } from "../../constants/constants";
import Action from "../../models/Action";
import IChampionStats, {
    getCarryScore,
    getKda,
    getWinrate,
} from "../../models/IChampionStats";
import Team from "../../models/Team";
import useStore from "../../store/DraftStore";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    championStats: IChampionStats;
    isPickBanned: boolean;
}

const ChampionStats: React.FC<IProps> = ({ championStats, isPickBanned }) => {
    const championData = useStore((store) => store.championData);

    const setPickBan = useStore((store) => store.setPickBan);

    const champion = championData[championStats.champion_id.toString()];

    const [_, drag] = useDrag(
        () => ({
            type: CHAMPION,
            item: { champion },
            canDrag: !isPickBanned,
            end: (item, monitor) => {
                const dropResult = monitor.getDropResult<{
                    team: Team;
                    type: Action;
                    index: number;
                }>();
                if (item && dropResult && !isPickBanned) {
                    setPickBan(
                        dropResult.type,
                        dropResult.team,
                        dropResult.index,
                        +item.champion.key
                    );
                }
            },
        }),
        [champion, isPickBanned]
    );

    const iconSize = isPickBanned
        ? "w-6 h-6 min-w-6 min-h-6"
        : "w-12 h-12 min-w-12 min-h-12";

    const padding = isPickBanned ? "px-6 py-2" : "px-6 py-4";

    return (
        <tr
            className={`${isPickBanned ? "opacity-20" : undefined} ${
                isPickBanned ? "" : "cursor-move"
            }`}
            ref={drag}
        >
            <td className={`${padding} whitespace-nowrap`}>
                <div className="flex items-center">
                    <div
                        className={`relative overflow-hidden mr-4  ${iconSize}`}
                    >
                        <img
                            src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                            className={`absolute inset-0 ${iconSize}`}
                            style={{ transform: "scale(1.1,1.1)" }}
                        ></img>
                    </div>
                    <a
                        href={`https://u.gg/lol/champions/${champion.id}/build`}
                        target="_blank"
                        rel="noreferrer"
                    >
                        <h4 className="text-xl font-header">{champion.name}</h4>
                    </a>
                </div>
            </td>
            <td className={`whitespace-nowrap text-right ${padding}`}>
                {getWinrate(championStats)}
            </td>
            <td className={`whitespace-nowrap text-right ${padding}`}>
                {championStats.games}
            </td>
            <td className={`whitespace-nowrap text-right ${padding}`}>
                {getKda(championStats)}
            </td>
            <td className={`whitespace-nowrap text-right ${padding}`}>
                {getCarryScore(championStats)}
            </td>
            <td
                className={`whitespace-nowrap text-right ${padding} text-primary`}
            >
                {(championStats.score * 100).toFixed(2)}
            </td>
            <td className={`whitespace-nowrap ${padding}`}>
                {championStats.recent_game ? "✓" : ""}
            </td>
        </tr>
    );
};

export default ChampionStats;
