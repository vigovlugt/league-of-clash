import BottomIcon from "../components/svg/roles/BottomIcon";
import JungleIcon from "../components/svg/roles/JungleIcon";
import MidIcon from "../components/svg/roles/MidIcon";
import SupportIcon from "../components/svg/roles/SupportIcon";
import TopIcon from "../components/svg/roles/TopIcon";
import Role from "../models/Role";

export function iconByRole(role: Role) {
    return {
        [Role.Adc]: BottomIcon,
        [Role.Jungle]: JungleIcon,
        [Role.Mid]: MidIcon,
        [Role.Supp]: SupportIcon,
        [Role.Top]: TopIcon,
        [Role.None]: null,
    }[role];
}
