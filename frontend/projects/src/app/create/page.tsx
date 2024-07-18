"use client"
import style from "./Register.module.css";
import {SubmitHandler, useForm} from "react-hook-form";
import TextBox from "@/components/form/TextBox/TextBox";
import TextBoxArea from "@/components/form/TextBoxArea/TextBoxArea";
import postCreateForm from "@/api/post";
type MovieCreationForm={
    movieTitle: string;
    movieContents: string;
    movieMemo:string;
};
export const Create = () =>{
    const form = useForm<MovieCreationForm>();
    const { handleSubmit } = form;
    const onSubmit : SubmitHandler<MovieCreationForm> = (submitForm : MovieCreationForm) =>{
        const {
            movieTitle,
            movieMemo,
            movieContents
        } = submitForm;
        const request = {
            "title":movieTitle,
            "description":movieContents,
            "memo":movieMemo,
            "session_token":"aaaaa", // 仮状態で適当な文字列に設定
        };
        const response = postCreateForm(request);
        console.log(response);
    }
    return (
        <main className={style["main"]}>
            <form className={style["create-form"]} onSubmit={handleSubmit(onSubmit)} >
                <TextBox title="映画タイトル" name="movieTitle" form={ form }/>
                <TextBoxArea title="映画情報" name="movieContents" form={ form }/>
                <TextBoxArea title="メモ" name="movieMemo" form={ form }/>
                <div className={style["create-form-btn"]}>
                    <button id="createBtn" type="submit">登録</button>
                </div>
            </form>
        </main>
    )
}
export default Create;
