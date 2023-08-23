import ReactDom from "react-dom"
import React, {FC} from "react"

type IQuoteModal = {
    openQuoteModal: boolean
    setOpenQuoteModal: (open: boolean) => void
    children: React.ReactNode
}

const QuoteModal: FC<IQuoteModal> = ({openQuoteModal, setOpenQuoteModal, children}) => {
    if (!openQuoteModal) return null
    return ReactDom.createPortal(
        <>
            <div className="fixed inset-0 bg-[rgba(0,0,0,.5)] z-[1000]" onClick={() => setOpenQuoteModal(false)}>
            </div>
            <div className="max-w-lg w-full rounded-md fixed top-0 xl:top-[10%] left-1/2 -translate-x-1/2 bg-white z-[1001] p-6">
            {children}
            </div>
        </>,
    document.getElementById("quote-modal") as HTMLElement
    )
}

export default QuoteModal