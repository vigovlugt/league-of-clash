interface IProps {
  className: string;
}

const TeamHeader: React.FC<IProps> = ({ className, children }) => (
  <div className={"relative " + className}>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 1196 25"
      fill="currentColor"
    >
      <path d="M1196,8h-149.5l-6.5-8H696l-9,8h-19l-6,5h19.3L670,23h-17l-3,2h22l13.6-12h363.8l8.9,9.5l-7.7-9.5H1183L1196,8z M691.2,8l6.8-6h341l5.6,6H691.2z"></path>{" "}
      <path d="M526,23l-11.3-10H534l-6-5h-19l-9-8H156l-6.5,8H0l13,5h132.5l-7.5,9.3l9.2-9.3h363.8L525,25h21l-3-2H526z M152.1,8l5.9-6h340l7,6H152.1z"></path>
    </svg>

    <div className="absolute left-1/2 transform -translate-x-1/2 bottom-0">
      {children}
    </div>
  </div>
);

export default TeamHeader;
