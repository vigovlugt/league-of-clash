import React from "react";
import DraftContext from "../../context/DraftContext";
import IBan from "../../models/IBan";

const DDRAGON_URL = process.env.NEXT_PUBLIC_DDRAGON_URL;

interface IProps {
    ban: IBan;
}

const Ban: React.FC<IProps> = ({ ban }) => {
    const { championData } = React.useContext(DraftContext);

    const banIds = ban.champion_ids.map((id) => championData[id.toString()].id);

    return (
        <div className="bg-dark rounded-lg p-4 flex justify-between items-center mb-4">
            <div className="flex">
                {banIds.map((id) => (
                    <div
                        className="relative w-12 h-12 min-w-12 min-h-12 overflow-hidden mr-4"
                        key={id}
                    >
                        <img
                            src={`${DDRAGON_URL}cdn/11.11.1/img/champion/${id}.png`}
                            className="w-12 h-12 min-w-12 min-h-12 absolute inset-0"
                            style={{ transform: "scale(1.1,1.1)" }}
                        ></img>
                    </div>
                ))}
            </div>
            <p className="text-xl text-gray-50">
                {(ban.priority * 100).toFixed(2)}
            </p>
        </div>
    );
};

export default Ban;