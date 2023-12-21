import { useEffect, useState } from "react"
import { Message } from "../components";
import { invoke } from "@tauri-apps/api";
import { Event, listen } from "@tauri-apps/api/event";

export default function Pubsub() {
  const [topic, setTopic] = useState("");
  const [errTopic, setErrTopic] = useState<string | null>(null);
  const [messages, setMessages] = useState<string[]>([]);
  const [message, setMessage] = useState("");

  useEffect(() => {
    invoke("start_consumer", { topic })
      .then(() => setErrTopic(null))
      .catch((err) => setErrTopic(err));
    const unlisten = listen('kafka_message', (event: Event<string>) => {
      setMessages(current => [...current, event.payload]);
    });

    return () => {
      invoke("stop_consumer");
      unlisten.then(f => f());
    }
  }, []);

  useEffect(() => {
    invoke("stop_consumer").then(() => {
      invoke("start_consumer", { topic })
        .then(() => setErrTopic(null))
        .catch((err) => setErrTopic(err));
    });
  }, [topic]);

  const produce = async () => {
    await invoke("produce", { message });
    setMessage("");
  }

  const handleForm = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    produce();
  }

  const ctrlEnter = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.ctrlKey && e.key === 'Enter') {
      produce();
    }
  }

  return (
    <section
      id="producer"
      className="p-4 w-4/5 h-screen bg-slate-300 flex flex-col gap-2"
    >
      <input
        type="text"
        className={`px-4 py-1 w-full h-[6%] rounded-md ${errTopic ? "border-red-500 border-2" : ""}`}
        name="topic"
        id="topic"
        placeholder="Topic..."
        onChange={(e) => setTopic(e.target.value)}
        value={topic}
      />

      <div className="flex gap-4 h-[94%]">
        <form className="flex flex-col gap-4 w-1/4 h-full" onSubmit={handleForm}>
          <textarea
            className="px-4 py-2 rounded-md h-full resize-none"
            name="message"
            id="message"
            placeholder="Message..."
            onChange={(e) => setMessage(e.target.value)}
            onKeyDown={ctrlEnter}
            value={message}
          ></textarea>

          <button
            type="submit"
            className="bg-blue-500 text-white font-bold px-4 py-2 rounded-md"
          >
            Publish Message
          </button>
        </form>

        <aside className="w-3/4 rounded-md p-4 bg-blue-500 flex flex-col gap-2">
          <div className="w-full h-full overflow-y-scroll rounded-sm p-2 flex flex-col gap-2 bg-slate-200 text-black">
            {messages.map((message, index) => (
              <Message key={index}>{message}</Message>
            ))}
          </div>
        </aside>
      </div>
    </section>
  );
}