import { Route, Routes } from "react-router-dom";

import { Sidebar } from "./components";
import { Dashboard, Pubsub } from "./pages";

function App() {
  return (
    <section id="app" className="flex w-screen h-screen bg-slate-300">
      <Sidebar />
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/pubsub" element={<Pubsub />} />
      </Routes>
    </section>
  );
}


export default App;
