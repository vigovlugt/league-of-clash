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
}

const ChampionStats: React.FC<IProps> = ({ championStats }) => {
    const championData = useStore((store) => store.championData);

    const allyBans = useStore((store) => store.allyBans);
    const enemyBans = useStore((store) => store.enemyBans);
    const allyPicks = useStore((store) => store.allyPicks);
    const enemyPicks = useStore((store) => store.enemyPicks);
    const pickBans = useStore((store) => store.getPickBannedChampions)();
    const setPickBan = useStore((store) => store.setPickBan);

    const isPickBanned = pickBans.includes(
        championStats.champion_id.toString()
    );

    const champion = championData[championStats.champion_id.toString()];

    const [_, drag] = useDrag(() => ({
        type: CHAMPION,
        item: { champion },
        end: (item, monitor) => {
            const dropResult =
                monitor.getDropResult<{
                    team: Team;
                    type: Action;
                    index: number;
                }>();
            if (item && dropResult) {
                console.log(dropResult, item);
                setPickBan(
                    dropResult.type,
                    dropResult.team,
                    dropResult.index,
                    item.champion.key
                );
            }
        },
    }));

    return (
        <tr className={isPickBanned ? "opacity-20" : undefined}>
            <td className={`px-6 py-4 whitespace-nowrap`}>
                <div className="flex items-center">
                    <div
                        ref={drag}
                        className="relative w-12 h-12 min-w-12 min-h-12 overflow-hidden mr-4 cursor-move"
                    >
                        <img
                            src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                            className="w-12 h-12 min-w-12 min-h-12 absolute inset-0"
                            style={{ transform: "scale(1.1,1.1)" }}
                        ></img>
                    </div>
                    <a
                        href={`https://u.gg/lol/champions/${champion.id}/build`}
                        target="_blank"
                    >
                        <h4 className="text-xl font-header">{champion.name}</h4>
                    </a>
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
