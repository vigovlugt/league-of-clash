import React from "react";
import IChampion from "../models/IChampion";

const DraftContext = React.createContext<{
    championData: { [key: string]: IChampion };
}>({
    championData: {},
});

export default DraftContext;
