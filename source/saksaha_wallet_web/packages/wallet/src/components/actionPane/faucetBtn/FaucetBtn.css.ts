import { style } from '@vanilla-extract/css';

export const InputWalletString = "  [1] Input your ID (secret key, nickname, etc...)";
export const InputData = "  [2] Select Coin, MRS Slots, and input your Data";

export const faucet_btn = style({
  boxSizing: "border-box",
  border: "3px solid green",

  textAlign: "center",
  fontSize: "18px",

  width: "80%",
  height: "20%",

  background: "white",
  color: "black",

  marginBottom: "20px",

  ":hover": {
    color: "black",
    background: "ivory",
    border: "3px solid red",
    cursor: "pointer",
  }
});

