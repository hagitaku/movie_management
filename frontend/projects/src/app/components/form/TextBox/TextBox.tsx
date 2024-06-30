import style from "./TextBox.module.css";

type TextBoxProps = {
    title: string;
    name: string;
}
const TextBox = (props : TextBoxProps) =>{
    const{
        title,
        name
    } = props;
    return(
        <div className={style.form_group}> 
            <label htmlFor={name}>{ title }</label>
            <div className={style.form_input}>
                <input id={name} name={name} type="text"></input>
            </div>
        </div>
    )
}
export default TextBox;