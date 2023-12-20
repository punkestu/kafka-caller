import { useState } from "react";
import { useNavigate } from "react-router-dom";
import SidebarButton from "./SidebarButton";

export default function Sidebar() {
  const navigate = useNavigate();
  const [pageState, setPageState] = useState("");

  const handleClick = (dir: string) => () => {
    setPageState(dir);
    navigate(`/${dir}`);
  }

  return <aside id="sidebar" className="flex flex-col w-1/5 h-screen bg-blue-500 p-4 text-slate-300">
    <h1 className="text-white font-black text-3xl mb-4">Kafka Caller</h1>
    <SidebarButton onClick={handleClick("")} isActive={pageState === ""}>Dashboard</SidebarButton>
    <SidebarButton onClick={handleClick("pubsub")} isActive={pageState === "pubsub"}>Pub/Sub</SidebarButton>
    <SidebarButton onClick={handleClick("comingsoon")} isActive={pageState === "comingsoon"}>Coming Soon</SidebarButton>
  </aside>;
}