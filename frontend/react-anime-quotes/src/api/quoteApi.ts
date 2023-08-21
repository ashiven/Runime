import axios from "axios"
import {
   CreateQuoteInput,
   UpdateQuoteInput,
} from "../components/quotes/create.quote"
import { IQuote, IQuoteResponse } from "./types"

const BASE_URL = "http://localhost:8080/api/"

export const quoteApi = axios.create({
   baseURL: BASE_URL,
   withCredentials: true,
})

quoteApi.defaults.headers.common["Content-Type"] = "application/json"

export const createQuoteFunction = async (quote: CreateQuoteInput) => {
   const response = await quoteApi.post<IQuoteResponse>("create/", quote)
   return response.data
}

export const retrieveQuoteFunction = async () => {
   const response = await quoteApi.get<IQuoteResponse>("random/")
   return response.data
}

export const updateQuoteFunction = async (
   quoteId: number,
   quote: UpdateQuoteInput
) => {
   const response = await quoteApi.patch<IQuoteResponse>(
      `update/${quoteId}`,
      quote
   )
   return response.data
}

const deleteQuoteFunction = async (quoteId: number) => {
   return quoteApi.delete<null>(`delete/${quoteId}`)
}
