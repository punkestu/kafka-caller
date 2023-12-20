import { useState } from "react";

export default function Dashboard() {
  const [broker, setBroker] = useState("");
  const [activeBroker, setActiveBroker] = useState("");

  const sendBroker = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setActiveBroker(broker);
    setBroker("");
  }

  return (
    <section className="w-4/5 h-full flex flex-col justify-center items-center">
      <form className="flex flex-col gap-4 w-1/3" onSubmit={sendBroker}>
        <input
          type="text"
          className="px-4 py-2 rounded-md"
          name="broker"
          id="broker"
          placeholder="Broker Address"
          onChange={(e) => setBroker(e.target.value)}
          value={broker}
        />
        <button type="submit" className="bg-blue-500 text-white font-bold px-4 py-2 rounded-md">
          Change broker
        </button>
      </form>
      <p className="mt-2">Active broker: {
        activeBroker ?
          <span className="text-green-300 bg-green-700">{activeBroker}</span> :
          <span className="text-red-500">N\A</span>
      }</p>
    </section>
  )
}