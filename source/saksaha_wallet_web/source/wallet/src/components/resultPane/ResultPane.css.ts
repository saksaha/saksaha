
import { style } from '@vanilla-extract/css';

export const asdf = style({
  width: "100%",
});

export const coin_record = style({
  border: "1px solid green",

  borderRadius: "5px",

  width: "fit-content",
  height: "fit-content",

  textAlign: "left",

  fontSize: "14px",

  fontWeight: "bold",

  marginTop: "10px",

  ":hover": {
    border: "1px solid red",

    background: "gainsboro"
  }
});

export const result = style({
  width: "100%",
  height: "100%",

  background: "skyblue",
  border: "3px solid orange",

  marginBottom: "30px",

});

export const result_body = style({
  height: "100%",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
});

