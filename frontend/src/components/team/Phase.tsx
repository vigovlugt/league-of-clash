import React from "react";

interface IProps {
    name: string;
    active?: boolean;
    onCLick?: Function;
}

export const Phase: React.FC<IProps> = ({ name, active }) => {
    return (
        <div
            className={`pl-4 flex items-center relative ${
                active ? "bg-primary text-dark" : ""
            }`}
        >
            {active && (
                <svg
                    height="64"
                    width="16"
                    fill="currentColor"
                    className="text-primary absolute -left-4"
                >
                    <polygon points="0,0 16,32, 16,0" />
                    <polygon points="16,32 0,64 16,64" />
                </svg>
            )}

            <span className="text-lg mr-2 font-header uppercase tracking-wider">
                {name}
            </span>
            <svg
                height="64"
                width="16"
                fill="currentColor"
                className="text-dark"
            >
                <polygon points="0,0 16,32, 16,0" />
                <polygon points="16,32 0,64 16,64" />
            </svg>
        </div>
    );
};
