import React from "react";
import IChampion from "../../models/IChampion";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    champion: IChampion;
    onClick: () => void;
}

const ChampionOption: React.FC<IProps> = ({ champion, onClick }) => {
    return (
        <div
            className="flex items-center px-3 py-2 cursor-pointer"
            onClick={onClick}
        >
            <div className="relative w-12 h-12 min-w-12 min-h-12 overflow-hidden  mr-4">
                <img
                    src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${champion.id}.png`}
                    className="w-12 h-12 min-w-12 min-h-12 absolute inset-0"
                    style={{ transform: "scale(1.1,1.1)" }}
                ></img>
            </div>
            <h4 className="text-xl font-header">{champion.name}</h4>
        </div>
    );
};

export default ChampionOption;
