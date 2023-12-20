export default function SidebarButton(opt: { isActive: boolean, onClick: () => void, children: React.ReactNode }) {
      return (
            <button onClick={opt.onClick} className={
                  `px-4 py-2 text-left ${opt.isActive ? "bg-blue-400 text-slate-50 rounded-xl" : ""}`
            }>{opt.children}</button>
      )
}