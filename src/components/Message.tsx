export default function Message(opt: {children: React.ReactNode}) {
      return (
            <div className="px-4 py-2 bg-slate-300 rounded-xl">{opt.children}</div>
      )
}