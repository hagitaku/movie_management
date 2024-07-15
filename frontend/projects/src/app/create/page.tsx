"use client"
import style from "./Register.module.css";
import {useForm} from "react-hook-form";
import TextBox from "../components/form/TextBox/TextBox";
import TextBoxArea from "../components/form/TextBoxArea/TextBoxArea";
import axios from "axios";

type MovieCreationForm={
    movieTitle: string;
    movieContents: string;
    movieMemo:string;
};
export const Create = () =>{
    const form = useForm<MovieCreationForm>();
    const { handleSubmit } = form;
    const postCreateForm = async (formData : MovieCreationForm) =>{
        const{
            movieTitle,
            movieContents,
            movieMemo,
        }=formData;
        console.log("Title:",movieTitle);
        console.log("Contents:",movieContents);
        console.log("Memo:",movieMemo);
        try{
            const BASE_API_URL = "http://backend:8080/";
            const REGISTER_API_PATH = "movie/register"
            const postUrl = BASE_API_URL+REGISTER_API_PATH;
            const response = await axios.post(postUrl,{
                "title":movieTitle,
                "description":movieContents,
                "memo":movieMemo,
            });
            console.log("Post Success:",response.data);
        }catch(error){
            console.log("Post error:",error);
        }
    }
    return (
        <main className={style.main}>
            <form className={style.form_group} onSubmit={handleSubmit(postCreateForm)}>
                <TextBox title="映画タイトル" name="movieTitle" form={ form }/>
                <TextBoxArea title="映画情報" name="movieContents" form={ form }/>
                <TextBoxArea title="メモ" name="movieMemo" form={ form }/>
                <div className={style.form_btn}>
                    <button id="register_btn" type="submit">登録</button>
                </div>
            </form>
        </main>
    )
}
export default Create;
