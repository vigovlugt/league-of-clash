import React from "react";
import { DropTargetMonitor, useDrop } from "react-dnd";
import { CHAMPION } from "../../constants/constants";
import Action from "../../models/Action";
import Team from "../../models/Team";
import useStore from "../../store/DraftStore";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    team: Team;
    championId: number | null;
    index: number;
    onClick: () => void;
}

const BanPanel: React.FC<IProps> = ({ team, championId, index, onClick }) => {
    const championData = useStore((store) => store.championData);

    const champion = championId != null ? championData[championId] : null;

    const [{ canDrop, isOver }, drop] = useDrop(
        () => ({
            accept: CHAMPION,
            drop: () => ({ team, type: Action.Ban, index }),
            collect: (monitor: DropTargetMonitor) => ({
                isOver: monitor.isOver(),
                canDrop: monitor.canDrop(),
            }),
        }),
        [team, index]
    );

    const isActive = canDrop && isOver;

    return (
        <div
            ref={drop}
            className={`my-3 mx-1 ${
                index === 2 ? "mr-auto" : ""
            } cursor-pointer ${isActive ? "bg-gray-700" : ""}`}
            onClick={onClick}
        >
            {champion ? (
                <div className="relative w-16 h-16 min-w-16 min-h-16 overflow-hidden">
                    <img
                        src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                        className={`w-16 h-16 min-w-16 min-h-16 absolute inset-0 filter grayscale`}
                        style={{ transform: "scale(1.1,1.1)" }}
                    ></img>
                </div>
            ) : (
                <div className="w-16 h-16 min-w-16 min-h-16 bg-light-dark" />
            )}
        </div>
    );
};

export default BanPanel;
