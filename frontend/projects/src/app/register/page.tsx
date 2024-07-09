"use client"
import style from "./Register.module.css";
import {useForm} from "react-hook-form";
import TextBox from "../components/form/TextBox/TextBox";
import TextBoxArea from "../components/form/TextBoxArea/TextBoxArea";

type MovieCreateForm={
    movieTitle: string;
    movieContents: string;
    movieMemo:string;
};
export const Register = () =>{
    const form = useForm<MovieCreateForm>();
    const { handleSubmit } = form;
    const postRegisterForm = (formData : MovieCreateForm) =>{
        const{
            movieTitle,
            movieContents,
            movieMemo,
        }=formData;
        console.log("Title:",movieTitle);
        console.log("Contents:",movieContents);
        console.log("Memo:",movieMemo);
    }
    return (
        <main className={style.main}>
            <form className={style.form_group} onSubmit={handleSubmit(postRegisterForm)}>
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
export default Register;
