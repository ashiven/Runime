import { FC } from "react"
import { SubmitHandler, useForm } from "react-hook-form"
import { twMerge } from "tailwind-merge"
import { object, string, TypeOf } from "zod"
import { zodResolver } from "@hookform/resolvers/zod"
import { LoadingButton } from "../LoadingButton"
import { toast } from "react-toastify"
import { useMutation, useQueryClient } from "@tanstack/react-query"
import { createQuoteFunction } from "../../api/quoteApi"
import NProgress from "nprogress"

type ICreateQuoteProps = {
   setOpenQuoteModal: (open: boolean) => void
}

const createQuoteSchema = object({
   quote: string().min(1, "Quote is required"),
   category: string().min(1, "Category is required"),
   anime: string().min(1, "Anime is required"),
   character: string().min(1, "Character is required"),
})

export type CreateQuoteInput = TypeOf<typeof createQuoteSchema>

const CreateQuote: FC<ICreateQuoteProps> = ({ setOpenQuoteModal }) => {
   const methods = useForm<CreateQuoteInput>({
      resolver: zodResolver(createQuoteSchema),
   })

   const {
      register,
      handleSubmit,
      formState: { errors },
   } = methods

   const queryClient = useQueryClient()

   const { mutate: createQuote } = useMutation({
      mutationFn: (quote: CreateQuoteInput) => createQuoteFunction(quote),
      onMutate() {
         NProgress.start()
      },
      onSuccess(data) {
         queryClient.invalidateQueries(["getQuotes"])
         setOpenQuoteModal(false)
         NProgress.done()
         toast("Quote create successfully", {
            type: "success",
            position: "top-right",
         })
      },
      onError(error: any) {
         setOpenQuoteModal(false)
         NProgress.done()
         const resMessage =
            error.response.data.message ||
            error.response.data.detail ||
            error.message ||
            error.toString()
         toast(resMessage, {
            type: "error",
            position: "top-right",
         })
      },
   })

   const onSubmitHandler: SubmitHandler<CreateQuoteInput> = async (data) => {
      createQuote(data)
   }

   return (
      <section>
         <div className="flex justify-between items-center mb-3 pb-3 border-b border-gray-200">
            <h2 className="text-2xl text-ct-dark-600 font-semibold">
               Create Quote
            </h2>
            <div
               onClick={() => setOpenQuoteModal(false)}
               className="text-2xl text-gray-400 hover:bg-gray-200 hover:text-gray-900 rounded-lg p-1.5 ml-auto inline-flex items-center cursor-pointer"
            >
               <i className="bx bx-x"></i>
            </div>
         </div>
         <form className="w-full" onSubmit={handleSubmit(onSubmitHandler)}>
            <div className="mb-2">
               <label
                  className="block text-gray-700 text-lg mb-2"
                  htmlFor="quote"
               >
                  Quote
               </label>
               <input
                  className={twMerge(
                     `appearance-none border border-gray-400 rounded w-full py-3 px-3 text-gray-700 mb-2  leading-tight focus:outline-none`,
                     `${errors["quote"] && "border-red-500"}`
                  )}
                  {...methods.register("quote")}
               />
               <p
                  className={twMerge(
                     `text-red-500 text-xs italic mb-2 invisible`,
                     `${errors["quote"] && "visible"}`
                  )}
               >
                  {errors["quote"]?.message as string}
               </p>
            </div>
            <div className="mb-2">
               <label
                  className="block text-gray-700 text-lg mb-2"
                  htmlFor="title"
               >
                  Category
               </label>
               <textarea
                  className={twMerge(
                     `appearance-none border border-gray-400 rounded w-full py-3 px-3 text-gray-700 mb-2 leading-tight focus:outline-none`,
                     `${errors.category && "border-red-500"}`
                  )}
                  rows={6}
                  {...register("category")}
               />
               <p
                  className={twMerge(
                     `text-red-500 text-xs italic mb-2`,
                     `${errors.category ? "visible" : "invisible"}`
                  )}
               >
                  {errors.category && errors.category.message}
               </p>
            </div>
            <div className="mb-2">
               <label
                  className="block text-gray-700 text-lg mb-2"
                  htmlFor="title"
               >
                  Anime
               </label>
               <textarea
                  className={twMerge(
                     `appearance-none border border-gray-400 rounded w-full py-3 px-3 text-gray-700 mb-2 leading-tight focus:outline-none`,
                     `${errors.anime && "border-red-500"}`
                  )}
                  rows={6}
                  {...register("anime")}
               />
               <p
                  className={twMerge(
                     `text-red-500 text-xs italic mb-2`,
                     `${errors.anime ? "visible" : "invisible"}`
                  )}
               >
                  {errors.anime && errors.anime.message}
               </p>
            </div>
            <div className="mb-2">
               <label
                  className="block text-gray-700 text-lg mb-2"
                  htmlFor="title"
               >
                  Character
               </label>
               <textarea
                  className={twMerge(
                     `appearance-none border border-gray-400 rounded w-full py-3 px-3 text-gray-700 mb-2 leading-tight focus:outline-none`,
                     `${errors.character && "border-red-500"}`
                  )}
                  rows={6}
                  {...register("character")}
               />
               <p
                  className={twMerge(
                     `text-red-500 text-xs italic mb-2`,
                     `${errors.character ? "visible" : "invisible"}`
                  )}
               >
                  {errors.character && errors.character.message}
               </p>
            </div>
            <LoadingButton loading={false}>Create Quote</LoadingButton>
         </form>
      </section>
   )
}

export default CreateQuote
