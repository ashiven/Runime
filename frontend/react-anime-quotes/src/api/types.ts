export type IQuote = {
   id: number
   quote: string
   category: string
   anime: string
   character: string
}

export type IGenericResponse = {
   status: string
   message: string
}

export type IQuoteResponse = {
   status: string
   quote: IQuote
}
