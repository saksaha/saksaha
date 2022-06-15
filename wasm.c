export memory memory(initial: 17, max: 0);

global g_a:int = 1048576;
export global data_end:int = 1076996;
export global heap_base:int = 1077008;

table T_a:funcref(min: 88, max: 88);

data d_calledOptionunwraponaNoneval(offset: 1048576) =
  "called `Option::unwrap()` on a `None` valueassertion failed: idx < CAP"
  "ACITY/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/alloc/src"
  "/collections/btree/node.rsassertion failed: edge.height == self.height"
  " - 1\00\00K\00\10\00[\00\00\00\80\02\00\00\09\00\00\00K\00\10\00[\00\00"
  "\00\84\02\00\00\09\00\00\00assertion failed: src.len() == dst.len()K\00"
  "\10\00[\00\00\00\b6\06\00\00\05\00\00\00K\00\10\00[\00\00\00F\04\00\00"
  "\16\00\00\00K\00\10\00[\00\00\00\83\04\00\00\16\00\00\00assertion fail"
  "ed: edge.height == self.node.height - 1\00\00\00K\00\10\00[\00\00\00\9f"
  "\03\00\00\09\00\00\00/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/l"
  "ibrary/alloc/src/collections/btree/navigate.rs\00\98\01\10\00_\00\00\00"
  "?\02\00\00V\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00\01"
  "\00\00\00\00\00\00\00\01\00\00\00\03\00\00\00\04\00\00\00\04\00\00\00\04"
  "\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00called `Option::unwrap()`"
  " on a `None` value/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/libr"
  "ary/alloc/src/collections/btree/map/entry.rs\00k\02\10\00`\00\00\00Q\01"
  "\00\002\00\00\00a map\00\00\00\08\00\00\00\00\00\00\00\01\00\00\00\09\00"
  "\00\00\00\00\00\00\ff\ff\ff\ff\ff\ff\ff\ff\0c\00\00\00\0c\00\00\00\04\00"
  "\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00a Display implementation ret"
  "urned an error unexpectedly/rustc/fe5b13d681f25ee6474be29d748c65adcd91"
  "f69e/library/alloc/src/string.rs\00\00O\03\10\00K\00\00\00f\09\00\00\0e"
  "\00\00\00\10\00\00\00\00\00\00\00\01\00\00\00\11\00\00\00called `Resul"
  "t::unwrap()` on an `Err` value\00\0c\00\00\00\14\00\00\00\04\00\00\00\12"
  "\00\00\00\13\00\00\00\04\00\00\00\04\00\00\00\14\00\00\00source/sak_ct"
  "rt_validator/src/validator.rs\00\00\08\04\10\00*\00\00\00\10\00\00\009"
  "\00\00\00\08\04\10\00*\00\00\00\12\00\00\00<\00\00\00046885b904a8b8cdd"
  "17cc40078ed114214586f197a664d6aa33d4b46cc3b712afcdef3d4d808bc7843beaea"
  "9e1a4c5ddeea47cbd27ea1af5ca13719a2f42c39167Cannot serialize validators"
  ", err: \d6\04\10\00"\00\00\00\08\04\10\00*\00\00\00\1d\00\00\00\15\00\00"
  "\00validators\00\00\08\04\10\00*\00\00\00$\00\00\00/\00\00\00Cannot se"
  "rialize storage, err: \00,\05\10\00\1f\00\00\00\08\04\10\00*\00\00\00J"
  "\00\00\00\0d\00\00\00Cannot Deserialize HashMap from storage, err: \00"
  "\00d\05\10\00.\00\00\00\08\04\10\00*\00\00\00R\00\00\00\11\00\00\00\08"
  "\04\10\00*\00\00\00`\00\00\00\0d\00\00\00internal error: entered unrea"
  "chable code/home/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec82"
  "3/serde_json-1.0.81/src/ser.rs\00\00\00\e4\05\10\00Y\00\00\00;\06\00\00"
  "\12\00\00\00\e4\05\10\00Y\00\00\003\08\00\00;\00\00\00\e4\05\10\00Y\00"
  "\00\00=\08\00\007\00\00\00\t\r\n\f\b\\\"called `Option::unwrap()` on a"
  " `None` value/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/a"
  "lloc/src/collections/btree/navigate.rs\a9\06\10\00_\00\00\00\b8\00\00\00"
  "'\00\00\00serialize_value called before serialize_key/home/ubuntu/.car"
  "go/registry/src/github.com-1ecc6299db9ec823/serde_json-1.0.81/src/valu"
  "e/ser.rs\00\00C\07\10\00_\00\00\00\9d\01\00\00\1f\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\f0?\00\00\00\00\00\00$@\00\00\00\00\00\00Y@\00\00\00"
  "\00\00@\8f@\00\00\00\00\00\88\c3@\00\00\00\00\00j\f8@\00\00\00\00\80\84"
  ".A\00\00\00\00\d0\12cA\00\00\00\00\84\d7\97A\00\00\00\00e\cd\cdA\00\00"
  "\00 _\a0\02B\00\00\00\e8vH7B\00\00\00\a2\94\1amB\00\00@\e5\9c0\a2B\00\00"
  "\90\1e\c4\bc\d6B\00\004&\f5k\0cC\00\80\e07y\c3AC\00\a0\d8\85W4vC\00\c8"
  "Ngm\c1\abC\00=\91`\e4X\e1C@\8c\b5x\1d\af\15DP\ef\e2\d6\e4\1aKD\92\d5M\06"
  "\cf\f0\80D\f6J\e1\c7\02-\b5D\b4\9d\d9yCx\eaD\91\02(,*\8b E5\032\b7\f4\ad"
  "TE\02\84\fe\e4q\d9\89E\81\12\1f/\e7'\c0E!\d7\e6\fa\e01\f4E\ea\8c\a09Y>"
  ")F$\b0\08\88\ef\8d_F\17n\05\b5\b5\b8\93F\9c\c9F"\e3\a6\c8F\03|\d8\ea\9b"
  "\d0\feF\82M\c7raB3G\e3 y\cf\f9\12hG\1biWC\b8\17\9eG\b1\a1\16*\d3\ce\d2"
  "G\1dJ\9c\f4\87\82\07H\a5\\c3\f1)c=H\e7\19\1a7\fa]rHa\a0\e0\c4x\f5\a6Hy"
  "\c8\18\f6\d6\b2\dcHL}\cfY\c6\ef\11I\9e\C\f0\b7kFI\c63T\ec\a5\06|I\\a0\b4"
  "\b3'\84\b1Is\c8\a1\a01\e5\e5I\8f:\ca\08~^\1bJ\9ad~\c5\0e\1bQJ\c0\fd\dd"
  "v\d2a\85J0}\95\14G\ba\baJ>n\ddll\b4\f0J\ce\c9\14\88\87\e1$KA\fc\19j\e9"
  "\19ZK\a9=P\e21P\90K\13M\e4Z>d\c4KW`\9d\f1M}\f9Km\b8\04n\a1\dc/LD\f3\c2"
  "\e4\e4\e9cL\15\b0\f3\1d^\e4\98L\1b\9cp\a5u\1d\cfL\91af\87ir\03M\f5\f9?"
  "\e9\03O8Mr\f8\8f\e3\c4bnMG\fb9\0e\bb\fd\a2M\19z\c8\d1)\bd\d7M\9f\98:Ft"
  "\ac\0dNd\9f\e4\ab\c8\8bBN=\c7\dd\d6\ba.wN\0c9\95\8ci\fa\acN\a7C\dd\f7\81"
  "\1c\e2N\91\94\d4u\a2\a3\16O\b5\b9I\13\8bLLO\11\14\0e\ec\d6\af\81O\16\99"
  "\11\a7\cc\1b\b6O[\ff\d5\d0\bf\a2\ebO\99\bf\85\e2\b7E!P\7f/'\db%\97UP_\fb"
  "\f0Q\ef\fc\8aP\1b\9d6\93\15\de\c0PbD\04\f8\9a\15\f5P{U\05\b6\01[*QmU\c3"
  "\11\e1x`Q\c8*4V\19\97\94Qz5\c1\ab\df\bc\c9Ql\c1X\cb\0b\16\00R\c7\f1.\be"
  "\8e\1b4R9\ae\bamr"iR\c7Y)\09\0fk\9fR\1d\d8\b9e\e9\a2\d3R$N(\bf\a3\8b\08"
  "S\ada\f2\ae\8c\ae>S\0c}W\ed\17-sSO\\ad\e8]\f8\a7Sc\b3\d8bu\f6\ddS\1ep\c7"
  "]\09\ba\12T%L9\b5\8bhGT.\9f\87\a2\aeB}T}\c3\94%\adI\b2T\\f4\f9n\18\dc\e6"
  "Tsq\b8\8a\1e\93\1cU\e8F\b3\16\f3\dbQU\a2\18`\dc\efR\86U\ca\1ex\d3\ab\e7"
  "\bbU?\13+d\cbp\f1U\0e\d85=\fe\cc%V\12N\83\cc=@[V\cb\10\d2\9f&\08\91V\fe"
  "\94\c6G0J\c5V=:\b8Y\bc\9c\faVf$\13\b8\f5\a10W\80\ed\17&s\cadW\e0\e8\9d"
  "\ef\0f\fd\99W\8c\b1\c2\f5)>\d0W\ef]3s\b4M\04Xk5\00\90!a9X\c5B\00\f4i\b9"
  "oX\bb)\808\e2\d3\a3X*4\a0\c6\da\c8\d8X5AHx\11\fb\0eY\c1(-\eb\ea\CY\f1r"
  "\f8\a5%4xY\ad\8fv\0f/A\aeY\cc\19\aai\bd\e8\e2Y?\a0\14\c4\ec\a2\17ZO\c8"
  "\19\f5\a7\8bMZ2\1d0\f9Hw\82Z~$|7\1b\15\b7Z\9e-[\05b\da\ecZ\82\fcXC}\08"
  ""[\a3;/\94\9c\8aV[\8c\0a;\b9C-\8c[\97\e6\c4SJ\9c\c1[= \b6\e8\\03\f6[M\a8"
  "\e3"4\84+\0I\ce\95\a02a\|\dbA\bbH\7f\95\[R\12\ea\1a\df\ca\ysK\d2p\cb\00"
  "]WP\de\06M\fe4]m\e4\95H\e0=j]\c4\ae]-\acf\a0]u\1a\b58W\80\d4]\12a\e2\06"
  "m\a0\09^\ab|M$D\04@^\d6\db`-U\05t^\cc\12\b9x\aa\06\a9^\7fW\e7\16UH\df^"
  "\af\96P.5\8d\13_[\bc\e4y\82pH_r\eb]\18\a3\8c~_'\b3:\ef\e5\17\b3_\f1_\09"
  "k\df\dd\e7_\ed\b7\cbEW\d5\1d`\f4R\9f\8bV\a5R`\b1'\87.\acN\87`\9d\f1(:W"
  ""\bd`\02\97Y\84v5\f2`\c3\fco%\d4\c2&a\f4\fb\cb.\89s\ax}?\bd5\c8\91a\d6"
  "\\8f,C:\c6a\0c4\b3\f7\d3\c8\fba\87\00\d0z\84]1b\a9\00\84\99\e5\b4eb\d4"
  "\00\e5\ff\1e"\9bb\84 \ef_S\f5\d0b\a5\e8\ea7\a82\05c\cf\a2\e5ER\7f:c\c1"
  "\85\afk\93\8fpc2g\9bFx\b3\a4c\fe@BXV\e0\d9c\9fh)\f75,\10d\c6\c2\f3tC7D"
  "dx\b30R\14EydV\e0\bcfY\96\afd6\0c6\e0\f7\bd\e3dC\8fC\d8u\ad\18e\14sTN\d3"
  "\d8Ne\ec\c7\f4\10\84G\83e\e8\f91\15e\19\b8eax~Z\be\1f\eee=\0b\8f\f8\d6"
  "\d3"f\0c\ce\b2\b6\cc\88Wf\8f\81_\e4\ffj\8df\f9\b0\bb\ee\dfb\c2f8\9dj\ea"
  "\97\fb\f6f\86D\05\e5}\ba,g\d4J#\af\8e\f4ag\89\1d\ecZ\b2q\96g\eb$\a7\f1"
  "\1e\0e\ccg\13w\08W\d3\88\01h\d7\94\ca,\08\eb5h\0d:\fd7\caekhHD\feb\9e\1f"
  "\a1hZ\d5\bd\fb\85g\d5h\b1J\adzg\c1\0ai\afN\ac\ac\e0\b8@iZb\d7\d7\18\e7"
  "ti\f1:\cd\0d\df \aai\d6D\a0h\8bT\e0i\0cV\c8B\aei\14j\8fkz\d3\19\84Ijs\06"
  "YH \e5\7fj\08\a47-4\ef\b3j\0a\8d\858\01\eb\e8jL\f0\a6\86\c1%\1fk0V(\f4"
  "\98wSk\bbk21\7fU\88k\aa\06\7f\fd\dej\bek*do^\cb\02\f3k5=\0b6~\c3'l\82\0c"
  "\8e\c3]\b4]l\d1\c78\9a\ba\90\92l\c6\f9\c6@\e94\c7l7\b8\f8\90#\02\fdl#s"
  "\9b:V!2m\ebOB\c9\ab\a9fm\e6\e3\92\bb\16T\9cmp\ce;5\8e\b4\d1m\0c\c2\8a\c2"
  "\b1!\06n\8fr-3\1e\aa;n\99g\fc\dfRJqn\7f\81\fb\97\e7\9c\a5n\dfa\fa}!\04"
  "\dbn,}\bc\ee\94\e2\10ov\9ck*:\1bEo\94\83\06\b5\08bzo=\12$qE}\b0o\cc\16"
  "m\cd\96\9c\e4o\7f\\c8\80\bc\c3\19p\cf9}\d0U\1aPpC\88\9cD\eb \84pT\aa\c3"
  "\15&)\b9p\e9\944\9bos\efp\11\dd\00\c1%\a8#qV\14A1/\92XqkY\91\fd\ba\b6\8e"
  "q\e3\d7z\de42\c3q\dc\8d\19\16\c2\fe\f7qS\f1\9f\9br\fe-r\d4\f6C\a1\07\bf"
  "br\89\f4\94\89\c9n\97r\ab1\fa\eb{J\cdr\0b_|s\8dN\02s\cdv[\d00\e26s\81T"
  "r\04\bd\9als\d0t\c7"\b6\e0\a1s\04Ry\ab\e3X\d6s\86\a6W\96\1c\ef\0bt\14\c8"
  "\f6\ddquAt\18ztU\ce\d2ut\9e\98\d1\ea\81G\abtc\ff\c22\b1\0c\e1t<\bfs\7f"
  "\ddO\15u\0b\afP\df\d4\a3Jugm\92\0be\a6\80u\c0\08wN\fe\cf\b4u\f1\ca\14\e2"
  "\fd\03\eau\d6\feL\ad~B v\8c>\a0X\1eSTv/N\c8\ee\e5g\89v\bbazj\df\c1\bfv"
  "\15}\8c\a2+\d9\f3vZ\9c/\8bv\cf(wp\83\fb-T\03_w&2\bd\9c\14b\93w\b0~\ec\c3"
  "\99:\c8w\\9e\e74@I\few\f9\c2\10!\c8\ed2x\b8\f3T):\a9gx\a50\aa\b3\88\93"
  "\9dxg^Jp5|\d2x\01\f6\\ccB\1b\07y\823t\7f\13\e2<y1\a0\a8/L\0dry=\c8\92;"
  "\9f\90\a6yMzw\0a\c74\dcyp\ac\8af\fc\a0\11z\8cW-\80;\09Fzo\ad8`\8a\8b{z"
  "el#|67\b1z\7fG,\1b\04\85\e5z^Y\f7!E\e6\1a{\db\97:5\eb\cfP{\d2=\89\02\e6"
  "\03\85{F\8d+\83\dfD\ba{L8\fb\b1\0bk\f0{_\06z\9e\ce\85$|\f6\87\18FB\a7Y"
  "|\faT\cfk\89\08\90|8*\c3\c6\ab\0a\c4|\c7\f4s\b8V\0d\f9|\f8\f1\90f\acP/"
  "};\97\1a\c0k\92c}\0a=!\b0\06w\98}L\8c)\\c8\94\ce}\b0\f7\999\fd\1c\03~\9c"
  "u\00\88<\e47~\03\93\00\aaK\ddm~\e2[@JO\aa\a2~\dar\d0\1c\e3T\d7~\90\8f\04"
  "\e4\1b*\0d\7f\ba\d9\82nQ:B\7f)\90#\ca\e5\c8v\7f3t\ac<\1f{\ac\7f\a0\c8\eb"
  "\85\f3\cc\e1\7fcalled `Option::unwrap()` on a `None` value/rustc/fe5b1"
  "3d681f25ee6474be29d748c65adcd91f69e/library/alloc/src/collections/btre"
  "e/navigate.rs\00\00\8b\11\10\00_\00\00\00\ff\01\00\00/\00\00\00called "
  "`Option::unwrap()` on a `None` value/home/ubuntu/.cargo/registry/src/g"
  "ithub.com-1ecc6299db9ec823/serde_json-1.0.81/src/read.rs\00\00\00'\12\10"
  "\00Z\00\00\00\a1\01\00\00\14\00\00\00'\12\10\00Z\00\00\00\c6\01\00\00\13"
  "\00\00\00'\12\10\00Z\00\00\00\d5\01\00\000\00\00\00'\12\10\00Z\00\00\00"
  "\cb\01\00\00)\00\00\00'\12\10\00Z\00\00\00\cf\01\00\004\00\00\00'\12\10"
  "\00Z\00\00\00>\02\00\00%\00\00\00\01\01\01\01\01\01\01\01\01\01\01\01\01"
  "\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\01\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00'\12\10\00Z\00\00\00\a6\03\00\00/\00\00\00\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\01\02\03\04\05\06"
  "\07\08\09\ff\ff\ff\ff\ff\ff\ff\0a\0b\0c\0d\0e\0f\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\0a\0b\0c\0d\0e\0f"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff"
  "\ff\ff\ff\ff\ff\ff\ff\ff\ff\15\00\00\00\0c\00\00\00\04\00\00\00\16\00\00"
  "\00\15\00\00\00\0c\00\00\00\04\00\00\00\17\00\00\00\16\00\00\00\f4\14\10"
  "\00\18\00\00\00\19\00\00\00\1a\00\00\00\1b\00\00\00\1c\00\00\000001020"
  "3040506070809101112131415161718192021222324252627282930313233343536373"
  "8394041424344454647484950515253545556575859606162636465666768697071727"
  "37475767778798081828384858687888990919293949596979899nulltruefalse{}\0a"
  "fmt errorinternal error: entered unreachable code/home/ubuntu/.cargo/r"
  "egistry/src/github.com-1ecc6299db9ec823/serde_json-1.0.81/src/ser.rs\00"
  "\009\16\10\00Y\00\00\00;\06\00\00\12\00\00\00nulltruefalse"\t\r\n\f\b\"
  "\\"0123456789abcdef[],{}:  \0a,\0a: \00\00\009\16\10\00Y\00\00\003\08\00"
  "\00;\00\00\009\16\10\00Y\00\00\00=\08\00\007\00\00\00uuuuuuuubtnufruuu"
  "uuuuuuuuuuuuuuu\00\00"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00"\00\00\00\0c\00\00\00\04\00\00"
  "\00#\00\00\00$\00\00\00%\00\00\00a Display implementation returned an "
  "error unexpectedly/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/libr"
  "ary/alloc/src/string.rs\00\00O\18\10\00K\00\00\00f\09\00\00\0e\00\00\00"
  "/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/str/p"
  "attern.rs\00\ac\18\10\00O\00\00\00\e3\05\00\00\14\00\00\00\ac\18\10\00"
  "O\00\00\00\e3\05\00\00!\00\00\00\ac\18\10\00O\00\00\00\ef\05\00\00\14\00"
  "\00\00\ac\18\10\00O\00\00\00\ef\05\00\00!\00\00\00&\00\00\00\00\00\00\00"
  "\01\00\00\00\11\00\00\00assertion failed: self.is_char_boundary(new_le"
  "n)O\18\10\00K\00\00\00\b2\04\00\00\0d\00\00\00\ac\18\10\00O\00\00\00p\04"
  "\00\00\17\00\00\00recursion limit exceededunexpected end of hex escape"
  "trailing characterstrailing commalone leading surrogate in hex escapek"
  "ey must be a stringcontrol character (\u0000-\u001F) found while parsi"
  "ng a stringinvalid unicode code pointnumber out of rangeinvalid number"
  "invalid escapeexpected valueexpected identexpected `,` or `}`expected "
  "`,` or `]`expected `:`EOF while parsing a valueEOF while parsing a str"
  "ingEOF while parsing an objectEOF while parsing a list at line  column"
  " \00\00\00\ac\18\10\00\00\00\00\00d\1b\10\00\09\00\00\00m\1b\10\00\08\00"
  "\00\00Error(, line: , column: )\00\00\00\90\1b\10\00\06\00\00\00\96\1b"
  "\10\00\08\00\00\00\9e\1b\10\00\0a\00\00\00\a8\1b\10\00\01\00\00\00inva"
  "lid type: , expected \00\00\00\cc\1b\10\00\0e\00\00\00\da\1b\10\00\0b\00"
  "\00\00invalid type: null, expected \00\00\00\f8\1b\10\00\1d\00\00\00/h"
  "ome/ubuntu/.cargo/registry/src/github.com-1ecc6299db9ec823/serde_json-"
  "1.0.81/src/error.rs\00 \1c\10\00[\00\00\00\97\01\00\00\1e\00\00\00 \1c"
  "\10\00[\00\00\00\9b\01\00\00\09\00\00\00 \1c\10\00[\00\00\00\a2\01\00\00"
  "\1e\00\00\00 \1c\10\00[\00\00\00\ab\01\00\00'\00\00\00 \1c\10\00[\00\00"
  "\00\af\01\00\00)\00\00\00called `Option::unwrap()` on a `None` value/r"
  "ustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/alloc/src/collec"
  "tions/btree/navigate.rs\00\00\f7\1c\10\00_\00\00\00\94\00\00\00$\00\00"
  "\00'\00\00\00\04\00\00\00\04\00\00\00(\00\00\00)\00\00\00*\00\00\000.0"
  "\00\00\00\00\00\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00 \9a\99\99"
  "\99\99\99\99\99\99\99\99\99\99\99\99\19\15\aeG\e1z\14\aeG\e1z\14\aeG\e1"
  "z\14\de$\06\81\95C\8bl\e7\fb\a9\f1\d2Mb\10\96\d4\09h"lxz\a5,C\1c\eb\e2"
  "6\1a\abCn\86\1b\f0\f9a\84\f0h\e3\88\b5\f8\14"6X8I\f3\c7\b46\8d\ed\b5\a0"
  "\f7\c6\10j#\8d\c0\0eR\a6\87WH\af\bc\9a\f2\d7\1a\88O\d7f\a5A\b8\9f\df9\8c"
  "0\e2\8ey\15\07\a6\12\1fQ\01-\e6\b2\94\d6&\e8\0b.\11\a4\09Q\cb\81h\ae\d6"
  "\b7\ba\bd\d7\d9\df|\1b\ea:\a7\a24\ed\f1\de_\95dy\e1\7f\fd\15\bb\c8\85\e8"
  "\f6\f0'\7f\19\11\ea-\81\99\97\11\f8\0d\d6@\be\b4\0ce\c2\81vIh\c2%\1c\93"
  "q\de3\98\90p\ea\01\9b+\a1\86\9b\84\16C\c1~)\e0\a6\f3!\9b\15V\e7\9e\af\03"
  "\12751\0f\cd\d7\85i+\bc\89\d8\97\b2\d2\1c\f9\90Z?\d7\df7!\89\96\d4FF\f5"
  "\0e\17\fasH\ccE\e6_\e7\a0\abC\d2\d1]r\12]\86\0dz<=f\a54\ac\d2\b6O\c9\83"
  "\1d\b1\9e\d7\94c\97\1eQ]#B\92\0c\a1\9c\17\c1Ky\dd\82\df~\da}O\9b\0e\0a"
  "\b4\e3\12h\ac[b\d1\98d*\96\e5^\17\10 9\1eS\f0\e2\81\a7\e0\b6\eeDQ\b2\12"
  "@\b3-\18\a9&O\ceRM\92Xj\a7\8e\a8\99\c2W\13A\a4~\b0\b7{P'\aa\d8}\da\f5\d0"
  "\f2\1e4Pe\c0_\c9\a6R\bb\13\cb\ae\c4@\c2\18\90\a6\ea\99L\d4\eb\0e\c9\0f"
  "<\f26\9a\ce\13\80\0a\11\c3\adSy\b1A\19`P\be\f6\b0\1fg\08t\02\8b\dc-\c1"
  "gG\b3\a6\fe^Z\19R\a0)5o\b0$4\86\9f\c2\eb\feKH\14\db\19\ee\90\f2Y\1d\90"
  "\9e\7fh\89e\d69\10_)\b0\b4\1d\c3\fbL\972\a7\a8\d5#\f6\19\b2\baY]\b15\96"
  "=\ac[\1f\baw\e9\c4\14(b\e1}'^\ab\97VIL\fb\92\87\9d\10\0d\9dh\c9\d8\c9\ab"
  "\f2\f0\0ez\f8\b7\a5\95\1a>\17\ba:z\a1\bc[Zr.-\93\84D\15\cbE\fb.\c8\1a\ca"
  "\af\ae\8e\8b\8aB\9d\03\11E\09\92\b1\a6\f7\dc\b2J\e4x\aa\9d\fb8\1b\04\a1"
  "A\c1\eb\92}\f5n\83-U\b1/\c7\15\03\b4gg\89ud\c4X\9cWw'&l\11\d2\ec\a5\d8"
  "\db\88mm\f4\c6%\f2\0b=\e0\1b\db#\ebF\16\07\be\8a\c38\1e(\a3\fdL\16I\b6"
  "U\d2\11l\fen\9c`KSO1\d7\11\0e\8a\ef\b6O\13\97\b1`gE\85\18\82\8b\1c\a5\a1"
  "\bf\f8r\0f\ac'\1a\b9j7\ad\01\d6\16\1eN\99`\c2rV\b9\e1`U,$\ceD\12\95\16"
  "\c2\cd\03\1eW\f55\ce\bb\13m\e3:\1d\ab\ab\01\0b\03\18\ac*+\d8/v\8aOb\17"
  "V\894o\02\e0\bc\bbU\13\f3\c4n\0c\b5\12\89\a8\ed\b1\d0\cc\c7\92\ef\1e\b8"
  "\d4Jz\ee\1d\07\baW\8e@\0a\d3\db\f2K\93\10o\fb\f1\17\06\c8\dfq\00\d5\a8"
  "|\f5o\0f\daX\fc'\13\d6\0cf\e93\bb\a7\fa\bbL\b2)\8e`\a6\1e\11\d7\84\87)"
  "\fcR\95\c9\a3\8eT\0b\1a\85\18\0e\ac\d0\d2\ba\c9\a8\aa\07\83\d8vo\ae\9d"
  "\13\e3\ac\1a\1e^\dc\da\dd\a5\d1\c0W\b2\b0b\1fO\8aHKK\b0H~QA\9a\ac\8e\c0"
  "\1b\19\d9\a1\d3\d5\d5Ym\cb\da\cd\e1V\a53\16\14{\81\dcw\11{W<\e2\d7\e7\ab"
  "\ea\c2\11\10*\cf`Y\82^\f2\c66&\a6\ac\aa\04\b6\19\bb\a5\80Gh\18\f5k\c5Q"
  "\ebVU\9d\91\14\96\84\00\06\edy*#\d1\a7"\df\dd}t\10V\074\a3\e1\8f\dd\d1"
  "\81\0c\d11\96\fcS\1aEl\f6\e8\1as\e4\a74=\a7\f4D\fd\0f\15\9eV\f8S\e2(\1d"
  "S]\97R]j\97\d9\10bW\8d\b9\03\dba\eb.\f2P\95\10\bf\f5\1a\e8E\a4\c7\cfHN"
  "\bcX[\da\dd\a6e\91\15 k\83l\d9\d3qc\ad\e2\e1\17\1f\1eA\11\cd\11\9f\ad("
  "\86\1c\9fH\04\03\f3dc\9b\1b\0b\db\18\beSk\b0\e5\06\9d5\8f\1d\e9\15\16\a2"
  "\15G\cb\0f\89\f3\eakJ\91r\e4 \ab\117\bcqxL\db\b8DF\aa\1b\84m\01E\1c_c\c1"
  "\c6\d6\15\c7\03\05UI\03\be\9a\9d\16\19\e9\cdkE\de867w\07i\fe\ae\17\12\c1"
  "A\16F\a2c\c1VXXr\0e\97\b1\f2\1c\ceg\ab\d1\81\1c\01\dfy\13\f5q\12\8e(\17"
  "\a5\ecUA\ce\164\7fa\dc\90\c1\0e\d8\86\12nGV5}$ e\02\c7\e7h\e4\8c\a4\1d"
  "%9x\f70\1d\80\ea\01l\b9 \1d\d7\b6\17\84\fa,\f9\f3\b0\99\bb4#aM\17\ac\f8"
  "\129\f7G(SN\_T8h\15\f2\acZ\1e.,\d3\b9u\0b}\7fC`SD[\8aH\18X#\dc\c7\f7\d5"
  "0\99\cf\19\a96|;m\13&\d2\f9r\8c\89\b4\8e\b2\8f\0e\f1\f9+\15\1f\b8A.\8f"
  "\a3\07*r(\a6\0b\f4\c7\bc\dd\18\fa\9a\be\a5O9\bb\c1\86\1e\d6\\06\97\e4\13"
  "\f6\f70\09\19\c2^\9c\d70\f0\fa\d6$\d4\1f\f8_Z\07\14h\e5Iy\8d&/\df\83v\19"
  "`\e6\e1\05\10 Qn\c7\0aR\bf\e5\cf^\14\1a\85\81\d1\0c\80\da\f1\05o\0e\99"
  "\84\d9K\10\f5\d4h\82\14\00\c4O\d6\e4\e3\f4\a0\f5\12\1a+w\ed\01\aa\99i\d9"
  "\11\b7\1c\f7\b3\f7\db\14\bc\c5\8a\01\88\14\ee\adt\92\b0\c5\\f9\af\10,\09"
  "\deh\a6\ed|IT\ea\80o\94(\b3\1a$\d4\e4S\b8W\ca:\10U\9a\bfv \\15\83v\1dC"
  "`y;bs\aa\ae\ff^\80\16\11\9e\bd\c8\d1f\f5+\9d\b8\10\b12\cb3W\1b\7fdmAR\c4"
  "\bc}`\0d\f4\8e\a2\\df\15\cc\b6\8ag\dbi\fd\ca\e6=\c3\d8N}\7f\11\df\8awr"
  "\c5\0f/\ab\d7/\05\8e\e4.\ff\1b\80\d5\92[\04s\f2\88\ac\8cj>\1d\bfe\16fD"
  "BI\d0(\f5\d3V=U\98J\ff\ea\11\a3\a0\03BMA\88\b9W\95\bb\f3\102\ab\1c\e9\e6"
  "\02h\d7\cd9ayw\fc\c2@[\ef\16TR\02 yqa\e7-\f9\c9h\cd\15Y\12\86P\9d\99\8e"
  "\b5h\a5|[vt\15V[\1d\d2\a6J\e1>\91 Q\fd\15\c5\f6\ddD|\17\0e\1f\a2\1a\ff"
  "@M\a7\caD7\92\b1\d0\c9\12J\cbi\f7d\ce\ae\0b\11nXPO\b4\0f\1e;<\ee\c5P\d8"
  "\8b<\a7\f1ys?\90\0c\18\c9\c9\f17\day\09\ca\85\f4\c7\c22@=\13\dbB\e9\bf"
  "\f6\c2\a8\a9o\ba\0c\9e\b7f\c8\1e\e3\9b\ba\cc+\cfS!&\95p~,R\a0\18\82I\95"
  "p\89r\a9\1a\b8\dd&e\f0t\b3\13\9du\88\1a\0f\84u\f7\8c/>\08\e7\87\85\1f\17"
  "^\a0{r6\91_\0a&\98\06\ec\9f7\19\df\e4\19\96[\f8@\19\d5\84F\05\f0\7f,\14"
  "L\eaG\ab\af\c6\00\e1\107\05\d1\8c\99#\10G\dd?EL\a4g\ce\e7$\d5\b4G\8f\d2"
  "\19\06\b1\cc\9d\d6\e9R\d8\1f\b7\dd\c3\9fr\a8\148'\0aKE\ee\dby\19,~i\19"
  "\c2\86\10Y\d8\a9\11\a2\e3_)\8fF0\0f\8f6q\1az\13\bb\a7\81\1c\b3\ba\a5k\f3"
  "\d8\d8^'\15/\a9\95\ec\9a\e3(bQ\89\8f\ad\e0K\ec\10\17u\ef\e0\f78\0e\9d\e8"
  "\0eL\af\9a\ac\13\1by*Y\1a\93-\d8\b0Sr\d6%\e2V\a9\15.UGH\0f\bey\8d\dc\c1"
  "\de\b7\81ET\11|\bb\0b\da~\96\8f\15\94\9c\97\8c\cf\08\ba\1b\97/\d6\14\ff"
  "\11\a6wv\b0\df\d6rm.\16y\8c\deC\ff\a7Q\f9\91\f3\b2x\f5\bd\be\11\8e\ad\fd"
  "\d2\fe?\1c\c2\1c\ec\b7Z"cd\1c\d8\8adB23\b0\01\17\f0_\15\b5\b5\b6\16F\a2"
  "\83\9b\8e\c2Y\01\acY\e6\dd\90\c4+\12\a3\039_\17\04\f6\ce\ac\c2\a3\fc\1a"
  "\d4\12\1d\83\9c-L\aci^r\bd\9b\1c\caHCB\17\9c\e3\8a\d6\89T\18\f5\fd\e2\16"
  "\08\07i\9b\12\c6\05\ab\bd\0fT\8d\ee/k\f1\0c\d8t\c5\1d\05k"\ferv\d7\be\8c"
  ""\c1pF*\d1\17\04\bcN\cb(\c5\12\ff\d6Ng\8dk\bb\0d\13\a0\f9}xt;Q\cb$~\d8"
  "{\12_|\1eMa\fe\f9)\c9\0d\09\b71\ad\fcA\7fc\18\0a\81\cb\94!\d4\d7\a0\c5"
  "'$\ca4\cc\82\13w\cexT\cf\b9\bfgo\0cmC!\ad7\1f\f9q-\dd\a5\94\cc\1fYp\8a"
  "\cfMW\f9\18\c7\f4\bd}Q\dd\d6\7fz\f3\a1?>\ac\fa\13\0b\ee/\c9\e8.\be\ff\c3"
  "\b8\9c2\fdy\f7\1f\d6$\f3\a0 \bf1f6\fa\16\c2\fd\c7\92\19x\1d\\1a\1a\cc'"
  "\b8^\fb\ab\01\cblu\14`\e4|{\ae\09S\93\18\c9\bcg\a2\f0]\10\99\a0\94\c5\b0"
  "B\eb\1e\f4t\94?j\e7/\1a\e1\e6v\04'\02\89\e5\*\dd2\88\1f\f3\14\e7\eb+\9d"
  "\85\ce\a0\b7\b0\ee\b0(\a0\7f\c2\10\d8\df\dfaoJ\01Y\b4JNt3\cc\d0\1a\adL"
  "\e6\e7%\d5\cd\e0)\a2>\90\8f\d6s\15\f1\d6Q\86QwqM\ee\b4\cb\d9rx)\11\e8W"
  "\e9\d6\e8\be\e8{\b0T\ac\8f\84\8du\1b \13!\dfS2\ba\fcY\dd\89\0cj\a4\f7\15"
  "\80B\e7\18C(\c8c\aeJnp\ee\e9\92\11fj\d8'8\0d\0d\06\17\11J\1a\17C\1e\1c"
  "\eb!\ad\ec,\a4=k\12tn{\12\9c~\16VNW\bd\f0\1c\fe\88\db\X\fcA\e3\fe\11#J"
  "%b\b4\94\96A_a\8d`6\05\cb\1c\e9\d4\1d\e8)\aa\abg\7f\e7=M\f8\d0\08\17\87"
  "\dd\17 \bb!V\b92\b9d\d7\f9sm\12\a5\95\8cf+i#\c2\ea\c1:\f2\c2\ec{\1d\1d"
  "\de\d6\1e\89\ba\82\ce\bb4b[\02W\96\17\18\18\dfK\07b5\a5\fc\f6\b4\e2\01"
  "\ac\de\12Y\f3dy\d8\9c\88;\94\f1\8776\131\1e\e1\f5\83\c7FJm\fc\dcZ\06\c6"
  "\91B'\18\1a+\03\06\9fnW0\17\af\9e\d1\a7\9bR\13\90\de\d1<\cb}%\1a%\181\1c"
  "\a6\92\ea\1e@\e5\a70<\fe\1dH\b7yZ\e3\84\a8\bb\18\00Q\86\c0\c91K\d3\c5\c7"
  "\ae\82\9dS\c9\13\cd\b4\a3\cdB\e9\11R\09\a6\17\d1\c8\85\a8\1f\a4\90\1c>"
  "\02!\dbt\07\b8\df@:\9eS\19P\0dJ\cb\01\b4\15\f7\05`\19g\fb\e4B\14\a7\0a"
  "\08\09\9b)\de\f87\b3zR\fc\835\10\d7\dd\0c\a8\91B0\8eY\b8*\b7\939\ef\19"
  "\13K\0a \0e\02\8d>\e1\f9\ee\f8Ba\bf\14\0f<\08\80>\9b=e\e7\c7X\fa\9b\1a"
  "\99\10\e4,\0d\00d\f8\c8n\a5\0c\8e\90\f9\90\8e\1a\ea#\a4\99\e9\f9\d3\8b"
  "\b7\a3q@a\da>\15\bb\1cP\e1\ba\94\a9<\f9\82\f4\99\1a\15\ff\10+a\b3\9b\c4"
  "\bau\c7\8e\d1 \c3]\bb1\1b\89\1a)\16j\95\c4\d2\0b\0e\e7h\b1b\c1\15\a1{\ba"
  "\11\88w\d0\dbo>\1f\87'\82g\11\9b\92]\1c@\bf\80,\e6c\98>?\d0\d8\1bIu\e4"
  "I3\cc3\bdQ\b6Fe\ff\0cG\16\d4]Pn\8f\d6\8f\ca\a7^\05Q\ccp\d2\11S\c9\b3\e3"
  "KW\19D\d9\fdnN\ad\e7\83\1c\a9:\f6\82\09yG\03\e1\97%\a5\8a\ec\cf\16\ba\fb"
  "\c4h\d4`l\cf\80y\84\ean\f0?\12*\f9\07\0e\874z\e5\9a\f5\d3\10K\1a3\1d"\94"
  "9\0bl\90.Q\e2*C\da\08\15\\17\b5\a9\c7\d5\bc\a6\8b\da\81U\cf\e1\d3\10\b0"
  "\12\87\0f\d9".q\df\90\9cU\e5\02S\81\e6\1dl\0c\14O\8bZL\da\16\de\1d\cf\a8"
  "\9a\eb\17\8a\a3\a9\a5\a2{\a3\aex~\b1\a5 \e2"\13\a9\05\a9\a2j_\d2}'\97\b5"
  "\a2\9a6\9e\1eT\d1 \82\88\7f\db\97\1f\ac\f7N\15\92~\18w\a7\80\ce\06f|yL"
  "#\c6\d8\ddt\98\13\f1\0b\01\e4\0ap-\8f\adk\a3'\96TZ\1fZ\d6\00P\a2Y$\0c\be"
  "\ef\b5\1fx\10\15\19\15E\9a\d9\81\14\1dp\fe\f2\f7\b2\f9\d9\10\14wj{\14\9b"
  "C\17\c0\fe[\c6(.{\0d\10\f2C\92\ed\c4\05\f2\cc\ca,\0a\0e}+\af\19\c2\9c\0e"
  "\be\d07[\0ao\bd\a1q\ca"\8c\14\ce\e3>\cbs\f9H\08\8c\97\b4'\d5\1bp\10\b0"
  "\9fdx\ec[\0e\da\ac%T\0cU\f9L\1a\c0\7fP`\f0\af>{\bd\b7\a9\d6\10a\0a\153"
  "f@\80\f3\bf\cb\95\97,\ee\des\1a\d5\10Rp\cdfRf\ac\efXG\b0d\b9\90\ee\1a\db"
  "Y\a4\b8\0e\85#&Gl\f3\b6\fa\a6\8b\15I\ae\b6\93\d8\d0\82\1el#)_\95\85<\11"
  "u\b0\8a\1f\f4\1a\9e\fd\ac8\a8\fe\ee\08\94\1b\f7Y\d5\b2)\af\b1\97\bd\93"
  "\86\98%\07\10\16,{w\f5\ba%\8e\ac\97\dc\9e\13\1el\a6\11\13\c5X"+\09}z\bf"
  "-\fe\b8\c9y=\1cvj\adN\ef\a0\fda\ccW\cb`\a1\94\97\16\c5\ee\bd\0bY\1a\fe"
  "\e7\09\13\09\e7M\dd\12\12:\b1\fcE[]c\a6\dc\84\0e\d8\af\fb\ea\1c\c8\8d0"
  "k\afJ\1c\85\b0\d0>\13\f3b"\17\d4\d7&\bc\f2n\e3\d0&\da\cbu\c2\e8\81\12\86"
  "\8c\a4\c6\ea\17\9f\b4\d7)F\89\9d\a7\9c\1dkpP\05\ef\df\18*F\ee\04\a1\17"
  "\86\b0\17\89\f3\d9\9d%\b3\e0Tk\8b\9dMy\9e\f3\12tR\f6bo\eb\cd\87xE/|(\97"
  "R\1e]\a8^\82\bf"\0b\d3\c6j\bf\c9\86\12B\18\e4\b9Kh\cc\1b<\0f\9f\88\ff:"
  "\d2\0eh\13m)y@z,`\18\98\da\98\91\83\e4\0c\1f$!\943\c8V\b3F\13\e2\13\0e"
  "6\1d\d7\18\b6MC)\a0x\8f8\dc\b4\dc\a4\91J\df\13\8a\afk\a8f'\7fZ`!a\a1\82"
  "\aa\cb\1f\a2\bf\ef\b9\eb\852\15M\b4M\b4\9b\bbo\19N\99\8ca\89\d1\8e\aa="
  "\90\a4\f6\e2bY\14\0c\e1\d6\1a\a1\a7\d8\ee\ca\d9\b6+O\82G\10E\9b$^\9br'"
  "~\11\f6\8a\df\b1\03\0c\1a\04I\1d\18I\f5\85\fe\0d\f8;\19[i\d6\14\d0\a0J"
  "\13\d4]\9e\cb\a4\f9/\14|\87\ab\10M\01\11RS\c9c\df:\\e6\b9\f9\0b\ac\1aq"
  "g\dat\0f\a1\1c\19/\b0\1e\fb\faoV\15\c1RH*\d9\80\b0\ad%\c0K//\f3\11\114"
  "Q\0d\aa\8e4\e7\15\09\cd\12\b2~\ebO\1b\c4\0dq\ee>]\1f\abm\0a\0f(2\89\d9"
  "\15\9d\a4\8d\8be\17\19\bcW\08\0c (\d4z\11\94:|\12<\f2\f4,Y\0d\e0\cc\d9"
  "\b9\f7\1bC\95\96\db\fc\f4\c3\f0\e0=\b3p\e1\c7_\16\03\11\12\16\97]6Z\1a"
  "\cb\f5&\819\e6\11\04\e8\1c\f0$\fcV\90\90\de"\0b5\8f\a3\1c\d0\ec\e3\8c\1d"
  "0\df\d9\a6K\82\a2]?\e9\16\da#\83=\b1Y\7f\e1\eb\a2\ceN\b12T\12\98/\b5\c2"
  "\cbhy\d1}\e4N\84S\1d\e3-`\bf]5\d6S\94\a7dPr\03v\17\1c\8b\e6e\b1*x\a9v\ec"
  "\b6\a6\8e\cf\c4\12\faD\d7o\b5\aa&\0f\f1\13\8b\d7}\b2\07\1ebj\df\bf*"R?"
  "'Co\acd(\06\18N\88\7f\99\88N\dbe\1f\9c\f2\89P 8\13J\0d\cc(tJ\c5oe\93\ea"
  "\0f\b43\c0\1e;\a4\09\87\f6\a1jY\84\0f"s\f6\c2\99\18\96\b6\07l\f8\e7\ee"
  "\ad6\d9\b4\f5\915\ae\13VW\0c\e0\f3?~I$\f5\ba"\83"}\1fE\ac\d6L\f6\ffd\d4"
  "\e9\90\95\e8h\e80\19\d1\89x=\f8\ff\83C\eesD\edS '\14t\a1\93\97\c6\cc\9c"
  "\cf\f1\8f\03\f1\0fM\1f\10R\02\b9%\a4Ga\7f\1c\b3\05\e8\7f\ae\cb\19\0f5\c7"
  "\b7\e9\d2M\cc\16\\d1\ec\ff\f1\a2\14\d9\90\d2_!\0f\0b=\12\b0\da#3[\82\10"
  "\c1\e7P\99hK\abaP\b3*\06\85+j\1ag\b9@\14\ba\a2"N@\Ukj\bc!\15S\94\00\dd"
  "\94\e8N\0b\cdID\bc\ee\c9\e7\10Q\ed\00\c8\87\da\17\12H\a9\d3\c6Jv\0c\1b"
  "\da\bd\00\a0lHF\dbl\87\dck\d5\91\a3\15\afd\cdL\bd\06\05I\8a\9f\e3\ef\dd"
  "\a7O\11\b1:\e2z\c8\0a\08\a8C\ff8\e6/\a6\b2\1b\f4.\e8\fb9\a29Si\ff\93\1e"
  "\f3\84(\16]\f2\ec/\fb\b4\c7u\87\ff\0f\b2\f5\03\ba\11.\eaG\e6\91!\d9"?\ff"
  "\7f\b6"\d3\\1c\f2T\06\85A\81z\b5e\ff\ff\91\e8\a8\b0\16\f5C87\01\01b\c4"
  "\b723\db\86\ed&\12\ee\9f\f3\f1\01h6:Y\84\eb\91\a4\15\0b\1d\8b\19\f6'\9b"
  "\b9^\fb\e0i\bctP\11<\17\d6z^\86\e2\fa~/\e7\87c]@t\96\12V\91\fd\d6\d0\f7"
  "\97\e5q\d98b\cd\86\bd\1d\ab\da\cax\0d\93y\84\c1z-\e8=\d2\ca\17V\15o-qB"
  "a\d0\9a\c8\8a\861\a8\08\13""\18\afNjhM\91\da\aa=O@t\1e\e8\b4y\f2>\88S\a4"
  "\da\ae\88d?\00]\18\87]a(\ffl\dc\e9\aeXmP\cc\99}\13\a4\95h\0de\ae`\a9\e4"
  "\8dH\1az\/\1f\83D\ed=\b7\be\b3\ba\83q\a0\aea\b0\f2\186\9d\8a1,2\f6.6\c1"
  "\e6\be\e7Y\f5\13\f0aw\82\13\1d\bd\e4\89\9b\d7\97?\f6\ee\1fZN,5\a9}\ca\83"
  "\a1\af\df\df2\f8\8b\19\15\a5V\f7 \fe\a1\9c\e7\f2\b2L\c2\f9o\14\aa\1d\12"
  "\f9\b31\1bJ\b9(\8fp\9b\94Y\10\dd\95\b6\c1\ec\b5^C\f5\0d\e5\80\c5\ed(\1a"
  "J\de^\01W^\e55\c4\a4\1dg\04\8b\ed\14\d5\b1\18\01\ac~\b7\c4i\1d~R\d0\08"
  "\be\10"\b6Z\9by\97%\a1\0f/0\b7\b3\a7\c9\1a\81^\15Ia\ac\b7M\d9X\f3\f8\c2"
  "\1fn\15\9bKD\07\81#\c6\d7\ad\e0\f5\935\e6$\11+\ac\d3>\9b\05=YI4V\86"=n"
  "\1b\bc\89\dc\cb\15\9e\fd\e0m\c3\11\05\82\ca\f1\15c\a1\e3o\11\18\fe\b3$"
  "iA7\9b;\8e\11\d1\9b\d2\7f\b5Yc\86\07u5%\c5\c5\16\1c\0e\e3\0e3\91\14\e9"
  "\d1\d2\90\f7P7\9ex\16\0b\1c?\8f\dav\batu\0d\c6@,\18\fa\11x\c61\e5\90$\f7"
  "\ed\bbH\a3g\e0Y\c3\1c-\05[\b7@\1d,\8b\c9\d3\b5\1fM\ae\02\17$\04|_\cd}V"
  "o\d4\0f+\e6p\8bh\12\06m\c6\98H\c9\f0~\ed\b2\11=N\12t\1d\9f\bd\9e\e0\06"
  "\a1\c0\98W\c2\a7\fd\a4\0e\90\17\e6\caKM\d2\80\00Gy\9b\ec\caP\a5\d9\12\a2"
  "DyH\1d\ce\00\d8\8e\c5\adD\81\08)\1e\82\d0-m\17\d83\13?\d1W\9d\9a\d3 \18"
  "\ce\a6$$yF\f6\a8e\a7\acJ\15vM\13}\a4:\a0\8e=\bdto\a5zw\88V\e2\1edP\95\e6"
  ">1d]\8c\b7\fb\c5\06\12\b5\18\b7\a6\aa\eb\cb\8d\b6Jp,\96\d1k\0e\c4\13W\a4"
  "\aa\12\13\16$\11\1aG\f0\e8\12\17\a0\1f\df\e9\ee\0e\dcD\83\da\14l\f3SB\df"
  "L\19\80!\bf\d8|\9d\02\e2C#)Ch\7f=\143\812z\fd}hN6\1cT\cf\b921\10\b8\ce"
  "P\90\95\c9@J\bd\c6\b9K)Q\e8\19\c6\0b\a7\a6w\d43\081\d2\c7o\87\da\b9\14"
  "k\09\ec\1e\c6v)\a0\8d\0e\d3\bf\d2\ae\94\10\df\db\acd\a3WB\00I\17\b8\ff"
  "\1d~\87\1a\19\e3#\ea\b5\df\01\cd\a0\12`\99\b119\15\ae\b5\1c\88\91L\cep"
  "Mu\e6\ad'\8e\fa\10\e2U\94\a6\b5\ad\e3\1a\af\bbpI\0c}*\1b\e8wC\85\c4W\e9"
  "{\f2b\8d\07=\97\bb\15\87\f95\04jy\87\c9\8e\b5\0a\06d\dfb\11q\c2\bc\06\10"
  "\8f\a5u\e4\88w\d6le\d1\1b'5\cak\a6\a5\b7\f7\e9\d3\92\ab\f0\1dA\16\1f\c4"
  "\a1\bc\1e\1e\c6_\ee\0f\0fV\8d\b1\cd\11e\d3\02adc\a3\ff\16\b3\b1\89HO|\1c"
  "Q\dc\9bMP\1c\e92\df(\8e\d4\06\d9\c9\16\0e}Iqs\e3 \8f\b2 \d8v\05\14;\12"
  "|.\0f\82\85\05\9b~\ea\cdY\f1;S+\1d\ca\be\a5\01\9e7\af\cb\ee\d7G\f4/\dc"
  "U\17\a1\98\844K\f9X\09\bf\acl\c3\8c\16\ab\12\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\10\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\14\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\19\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00@\1f\00\00\00\00\00\00\00\00\00\00\00\00\00\00\88\13\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00j\18\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\80\84\1e\00\00\00\00\00\00\00\00\00\00\00\00\00\d0\12\13\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\84\d7\17\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00e\cd\1d\00\00\00\00\00\00\00\00\00\00\00\00 _\a0\12\00\00\00"
  "\00\00\00\00\00\00\00\00\00\e8vH\17\00\00\00\00\00\00\00\00\00\00\00\00"
  "\a2\94\1a\1d\00\00\00\00\00\00\00\00\00\00\00@\e5\9c0\12\00\00\00\00\00"
  "\00\00\00\00\00\00\90\1e\c4\bc\16\00\00\00\00\00\00\00\00\00\00\004&\f5"
  "k\1c\00\00\00\00\00\00\00\00\00\00\80\e07y\c3\11\00\00\00\00\00\00\00\00"
  "\00\00\a0\d8\85W4\16\00\00\00\00\00\00\00\00\00\00\c8Ngm\c1\1b\00\00\00"
  "\00\00\00\00\00\00\00=\91`\e4X\11\00\00\00\00\00\00\00\00\00@\8c\b5x\1d"
  "\af\15\00\00\00\00\00\00\00\00\00P\ef\e2\d6\e4\1a\1b\00\00\00\00\00\00"
  "\00\00\00\92\d5M\06\cf\f0\10\00\00\00\00\00\00\00\00\80\f6J\e1\c7\02-\15"
  "\00\00\00\00\00\00\00\00 \b4\9d\d9yCx\1a\00\00\00\00\00\00\00\00\94\90"
  "\02(,*\8b\10\00\00\00\00\00\00\00\00\b94\032\b7\f4\ad\14\00\00\00\00\00"
  "\00\00@\e7\01\84\fe\e4q\d9\19\00\00\00\00\00\00\00\880\81\12\1f/\e7'\10"
  "\00\00\00\00\00\00\00\aa|!\d7\e6\fa\e01\14\00\00\00\00\00\00\80\d4\db\e9"
  "\8c\a09Y>\19\00\00\00\00\00\00\a0\c9R$\b0\08\88\ef\8d\1f\00\00\00\00\00"
  "\00\04\be\b3\16n\05\b5\b5\b8\13\00\00\00\00\00\00\85\ad`\9c\c9F"\e3\a6"
  "\18\00\00\00\00\00@\e6\d8x\03|\d8\ea\9b\d0\1e\00\00\00\00\00\e8\8f\87+"
  "\82M\c7raB\13\00\00\00\00\00\e2si\b6\e2 y\cf\f9\12\18\00\00\00\00\80\da"
  "\d0\03d\1biWC\b8\17\1e\00\00\00\00\90\88b\82\1e\b1\a1\16*\d3\ce\12\00\00"
  "\00\00\b4*\fb"f\1dJ\9c\f4\87\82\17\00\00\00\00a\f5\b9\ab\bf\a4\\c3\f1)"
  "c\1d\00\00\00\a0\9T\cb\f7\e6\19\1a7\fa]\12\00\00\00\c8\b3G)\be\b5`\a0\e0"
  "\c4x\f5\16\00\00\00\ba\a0\99\b3-\e3x\c8\18\f6\d6\b2\1c\00\00@t\04@\90\fc"
  "\8dK}\cfY\c6\ef\11\00\00P\91\05P\b4{q\9e\C\f0\b7k\16\00\00\a4\f5\06d\a1"
  "\da\0d\c63T\ec\a5\06\1c\00\80\86Y\84\de\a4\a8\c8[\a0\b4\b3'\84\11\00 \e8"
  "o%\16\ce\d2\bar\c8\a1\a01\e5\15\00(\e2\cb\ae\9b\81\87i\8f:\ca\08~^\1b\00"
  "Ym?M\01\b1\f4\a1\99d~\c5\0e\1b\11@\afH\8f\a0A\ddq\0a\c0\fd\ddv\d2a\15\10"
  "\db\1a\b3\08\92T\0e\0d0}\95\14G\ba\1a\ea\c8\f0oE\db\f4(\08>n\ddll\b4\10"
  "$\fb\ec\cb\16\1223\8a\cd\c9\14\88\87\e1\14\ed9\e8~\9c\96\fe\bf\ec@\fc\19"
  "j\e9\19\1a4$Q\cf!\1e\ff\f7\93\a8=P\e21P\10Am%C\aa\e5\fe\f5\b8\12M\e4Z>"
  "d\14\92\c8\ee\d3\14\9f~3gW`\9d\f1M}\19\b6z\ea\08\daF^\00Am\b8\04n\a1\dc"
  "\1f\b2\8c\92EH\ec:\a0HD\f3\c2\e4\e4\e9\13\de/\f7VZ\a7I\c8Z\15\b0\f3\1d"
  "^\e4\18\d6\fb\b4\ec0\11\z\b1\1a\9cp\a5u\1d\1fe\1d\f1\93\be\8ay\ec\ae\90"
  "af\87ir\13\bfd\ed8n\ed\97\a7\da\f4\f9?\e9\03O\18\ef\bd(\c7\c9\e8}Q\11r"
  "\f8\8f\e3\c4b\1e\b5vy\1c~\b1\ee\d2JG\fb9\0e\bb\fd\12b\d4\97\a3\dd]\aa\87"
  "\1d\19z\c8\d1)\bd\17{\c9}\0cU\f5\94\e9d\9f\98:Ft\ac\1d\ed\9d\ce'U\19\fd"
  "\11\9fc\9f\e4\ab\c8\8b\12hE\c2q\aa_|\d6\86<\c7\dd\d6\ba.\17\c2\d62\0e\95"
  "w\1b\8c\a8\0b9\95\8ci\fa\1c9\c6\df(\bd*\91WI\a7C\dd\f7\81\1c\12\c8\b7\17"
  "sluu\ad\1b\91\94\d4u\a2\a3\16\ba\a5\dd\8f\c7\d2\d2\98b\b5\b9I\13\8bL\1c"
  "\94\87\ea\b9\bc\c3\83\9f]\11\14\0e\ec\d6\af\11y)e\e8\ab\b4d\07\b5\15\99"
  "\11\a7\cc\1b\16\d7s~\e2\d6\e1=I"[\ff\d5\d0\bf\a2\1bf\08\8fM&\ad\c6m\f5"
  "\98\bf\85\e2\b7E\11\80\ca\f2\e0oX8\c92\7f/'\db%\97\15 }/\d9\8bn\86{\ff"
  "^\fb\f0Q\ef\fc\1a4\ae\bdg\17\054\ad_\1b\9d6\93\15\de\10\c1\19\adA]\06\81"
  "\987bD\04\f8\9a\15\152`\18\92\f4G\a1~\c5zU\05\b6\01[\1a\1f<O\db\f8\cc$"
  "o\bblU\c3\11\e1x\10'\0b#\127\00\eeJ\ea\c7*4V\19\97\14\f0\cd\ab\d6D\80\a9"
  "\dd\e4y5\c1\ab\df\bc\19\b6`+\06+\f0\89\0a/l\c1X\cb\0b\16\10\e48\b6\c75"
  "l,\cd:\c7\f1.\be\8e\1b\14\1d\c7\a39C\87w\80\099\ae\bamr"\19\e4\b8\0c\08"
  "\14i\95\e0K\c7Y)\09\0fk\1f\8e\f3\07\85\aca]l\8f\1c\d8\b9e\e9\a2\13r\f0"
  "I\a6\17\batG\b3#N(\bf\a3\8b\18\8fl\dc\8f\9d\e8Q\19\a0\aca\f2\ae\8c\ae\1e"
  "\d9\c3\e9yb1\d3\0f\e4\0b}W\ed\17-\13\cf4d\18\bb\fd\c7\13\ddN\\ad\e8]\f8"
  "\17\03B}\de)\fd\b9X\94b\b3\d8bu\f6\1dBI\0e+:>t\b7\9c\1dp\c7]\09\ba\12\92"
  "\db\d1\b5\c8MQ\e5\03%L9\b5\8bh\17wRF\e3:\a1\a5\deD.\9f\87\a2\aeB\1d\8a"
  "\f3\0b\ce\c4\84'\0b\eb|\c3\94%\adI\12m\f0\8e\01\f6e\f1\cd%\\f4\f9n\18\dc"
  "\16\88\ac\f2\81s\bfmA/sq\b8\8a\1e\93\1c\d5\ab71\a8\97\e4\88\fd\e7F\b3\16"
  "\f3\db\11\ca\96\85=\92\bd\1d\eb\fc\a1\18`\dc\efR\16}\fc\e6\cc\f6,\e5%|"
  "\ca\1ex\d3\ab\e7\1b\ce]\10@\1a<\af\97\8d>\13+d\cbp\11Bu\14\d0 \0b\9b\fd"
  "0\0e\d85=\fe\cc\15\92\92\19\04\e9\cd\01=\bd\11N\83\cc=@\1b\9b\fb\8f\a2"
  "\b1 !F\16\cb\10\d2\9f&\08\11\82\fa3\0b\deh\a9\d7\db\fd\94\c6G0J\15#\f9"
  "\00\8e\15\c3\93\cdR=:\b8Y\bc\9c\1a\b6\9b\c0x\edY|\c0Sf$\13\b8\f5\a1\10"
  "\a3\c2\f0\d6hp\9b\b0\e8\7f\ed\17&s\ca\14L\f3\ac\0c\83L\c2\dc\e2\df\e8\9d"
  "\ef\0f\fd\19\0f\18\ec\e7\d1o\f9\c9\ed\8b\b1\c2\f5)>\10\13\1e\e7a\c6\cb"
  "w<\e9\ee]3s\b4M\14\98\e5`\fa\b7\be\95\8b\a3j5\00\90!a\19\fe\1e\f9\f8e."
  "{nL\c5B\00\f4i\b9\1f_\b3\9b\bb\ff\fc\0c\c5O\bb)\808\e2\d3\137\a0\82\aa"
  "?<P\b6#*4\a0\c6\da\c8\18DH#\95OK\e4\a3\ac4AHx\11\fb\1e+\0d6\bd\11\afn\e6"
  "\eb\c0(-\eb\ea\\13u\90\83,\d6Z\0a\e0&\f1r\f8\a5%4\18\93t\a4\b7\8b\f1\0c"
  "\98p\ad\8fv\0f/A\1e\dc\c8\c6R\f7\16\08_f\cc\19\aai\bd\e8\12\13{x'\b5\1c"
  "\ca\f6\7f?\a0\14\c4\ec\a2\17\d7\99Vq\e2\a3|\f4_O\c8\19\f5\a7\8b\1d& \d6"
  "\86m\e6\cd\f8\9b1\1d0\f9Hw\120\a8\8b\e8\08`\01\f7\02~$|7\1b\15\17<\92\ae"
  ""\0b\b8\c1\b4\83\9d-[\05b\da\1ce\1b\ad\f5\06\13\f9Pr\82\fcXC}\08\12?b\18"
  "\b3\c8W7\e5\0e\a3;/\94\9c\8a\16\cfz\de\df\ba-\85\9e\d2\8b\0a;\b9C-\1c\c1"
  "\0c\eb\cb\94<\13\a3c\97\e6\c4SJ\9c\11\f1\cf\e5\fe\b9\0b\d8\8b<= \b6\e8"
  "\\03\16\eeC\9f~\a8\0e\ce\ae\8bL\a8\e3"4\84\1bu\8a#O)\c9@M\d7/I\ce\95\a0"
  "2\11\12m\ec\a2s\fb\90 \cd{\dbA\bbH\7f\15V\88\a7\8bP:\b5h\c0ZR\12\ea\1a"
  "\df\1a6\b5HWrDqA\b8xsK\d2p\cb\10\83\e2\1a\ed\8e\95\cdQ\e6VP\de\06M\fe\14"
  "$\9ba\a8\f2\fa@\e6\9fl\e4\95H\e0=\1a\f7\00=\a9\d7\9c\e8\ef\e3\c3\ae]-\ac"
  "f\104A\8c\93\0d\c4\e2\eb\dct\1a\b58W\80\14\81Qo\f8\10u\db&\14\12a\e2\06"
  "m\a0\19\f1\92E\9b*)I\98L\ab|M$D\04\10\ad\f7\16Bus[\be\1f\d6\db`-U\05\14"
  "\98\b5\9c\92RP\f2\ad\a7\cb\12\b9x\aa\06\19\ff\e2C7g\e4n\99\91~W\e7\16U"
  "H\1f\dfm\8a\82\c0N\e5\ff\1a\af\96P.5\8d\13W\09-\a3p\a2\de\bf\e1Z\bc\e4"
  "y\82p\18\adK\f8\cb\0cK\d6/\9aq\eb]\18\a3\8c\1eL/{\ff\e7\ee\e5]\00'\b3:"
  "\ef\e5\17\13\1f\fbY\ff\a1j_u\c0\f0_\09k\df\dd\17\e7y0\7fJE\b7\92\f0\ec"
  "\b7\cbEW\d5\1d0L~\8fN\8b\b2[\16\f4R\9f\8bV\a5\12<\df]3".\9f\f2\1b\b1'\87"
  ".\acN\17\0bW5\c0\aa\f9F\efb\9d\f1(:W"\1dgV!\b8\0a\\8c\d5]\02\97Y\84v5\12"
  "\01\ac)f\0ds\efJ\f5\c2\fco%\d4\c2\16\01\17\b4\bf\d0O\ab\9d\b2\f3\fb\cb"
  ".\89s\1c`\8e\d0w\e2\11\8b\a2Ox}?\bd5\c8\11\f9\b1\c4\15[\d6-\8bc\d6\\8f"
  ",C:\16w\de5\db\f1K\f9m\fc\0b4\b3\f7\d3\c8\1b\0a\ab\01)w\cf\bb\c4}\87\00"
  "\d0z\84]\11\cd\15B\f3T\c3\ea5]\a9\00\84\99\e5\b4\15@\9b\120*te\83\b4\d3"
  "\00\e5\ff\1e"\1b\08\a1\0b^\9ah\1f\d2P\84 \ef_S\f5\10J\89\8e\f5\c0B\a7\06"
  "e\a5\e8\ea7\a82\15\9d+\f22q\13QH\be\ce\a2\e5ER\7f\1aB[\d7\bf&\ac2\ed6\c1"
  "\85\afk\93\8f\10\122\cdo0W\7f\a8\841g\9bFx\b3\14\97~\c0\8b\fc,\9f\d2\e5"
  "\fd@BXV\e0\19\1eOX\d7\1d|\a3\a3\af\9eh)\f75,\10\e6b.M%[\8c\8c[\c6\c2\f3"
  "tC7\14\9f\fby\a0\eeq\afo\f2w\b30R\14E\19\87z\98HjN\9b\0b\efU\e0\bcfY\96"
  "\1f\94L_m\02\11Ag\b55\0c6\e0\f7\bd\13\ba\1f\b7\08CU\11\c1"C\8fC\d8u\ad"
  "\18\a8\e7\e4\ca\93\aaUq\eb\13sTN\d3\d8\1e\c9\10\cf^\9c\8a\d5&s\ec\c7\f4"
  "\10\84G\13\fb\d4\82vC\ed\8a\f0\8f\e7\f91\15e\19\18:\8a#T\94\a8\ad\ecsa"
  "x~Z\be\1f\1ed6\96\b4\\89\ecs\e8<\0b\8f\f8\d6\d3\12\fd\c3\bb\e1\b3\ab\e7"
  "\90"\0c\ce\b2\b6\cc\88\17\fd\b4*\da\a0\96!5+\8f\81_\e4\ffj\1d\1e\b1Z\88"
  "$\fe4\01{\f9\b0\bb\ee\dfb\12e]q\aa\ad=\82\c1\d97\9dj\ea\97\fb\16\bf\b4"
  "\0d\15\19\cd\e21\d0\85D\05\e5}\ba\1c\f7\90(\ad/\c0-\1f\a2\d3J#\af\8e\f4"
  "\115\b5r\98;0\f9\a6\8a\88\1d\ecZ\b2q\16\82b\8f~J|\b7P\ad\ea$\a7\f1\1e\0e"
  "\1c\91\9d\19\8f\ae\adrR\ac\12w\08W\d3\88\11\f6\04\e02\1aY\0fgW\d7\94\ca"
  ",\08\eb\153\06\98\bf`/\d3@-\0d:\fd7\cae\1b\e0\03\bfw\9c\fd\83H<HD\feb\9e"
  "\1f\11\d8\c4\ae\95\03\fd\a4ZKZ\d5\bd\fb\85g\15\0ev\1a{D<N1\de\b0J\adzg"
  "\c1\1a\c9\89\f0\cc\aa\e5\d0\de\8a\aeN\ac\ac\e0\b8\10;\ac,\80\15\1f\85\96"
  "-Zb\d7\d7\18\e7\14J\d77\e0\daf&\fc\b8\f0:\cd\0d\df \1a\8e\e6"\ccH\00\98"
  "\9ds\d6D\a0h\8bT\102\a0+\ffZ\00\fe\84\10\0cV\c8B\aei\14>\88\f6\beq\80="
  "\a6\14\8fkz\d3\19\84\19N*\b4.\8e\e0\cc\cf\d9r\06YH \e5\1fp\9a0\ddX\0c\e0"
  "!\c8\07\a47-4\ef\13\0d\c1|\14o\0fX*\ba\09\8d\858\01\eb\18P\f1\9b\d9J\13"
  "\ee\b4(L\f0\a6\86\c1%\1f\d2v\01\c8\0e\cc\14q\99/V(\f4\98w\13\86\d4\01z"
  "\12\ffY\cd\7f\bbk21\7fU\18\a8I\82\18\d7~\b0\c0_\aa\06\7f\fd\dej\1e\09n"
  "QoFOn\d8{*do^\cb\02\13\8b\c9%\0b\18\e3\89\ce\1a5=\0b6~\c3\17\ee;\ef\0d"
  "\de[,\82a\82\0c\8e\c3]\b4\1du\85\b5\c8j\b9[\f1|\d1\c78\9a\ba\90\12\d2\e6"
  "\e2z\c5\a7\b2-\dc\c5\f9\c6@\e94\17\86\a0\9b\d9\b6Q\1f9S7\b8\f8\90#\02\1d"
  "TD\01H\12\93\b3\03\94"s\9b:V!\12i\95\01\da\d6w\a0\049\ebOB\c9\ab\a9\16"
  "\c3\fa\81\90\cc\95\c8E\07\e6\e3\92\bb\16T\1c\ba<Q\da\9f]\9d\8b\c4o\ce;"
  "5\8e\b4\11\e8\8b\e5\d0\07\b5\84\ae\b5\0b\c2\8a\c2\b1!\16\e3\ee\1e\c5I\e2"
  "%\1a\a3\8er-3\1e\aa\1bMU3\1bn\adW\f0%\99g\fc\dfRJ\11\a1*\00\a2\c9\98ml"
  "o\7f\81\fb\97\e7\9c\15I5\80\0a\fc\fe\88GK\dfa\fa}!\04\1bN!\90\86]\9f\b5"
  "\0c\8f+}\bc\ee\94\e2\10\a1)4\e84\07\e3\cfrv\9ck*:\1b\15\0a4A"\02\c9\db"
  "\83\0f\94\83\06\b5\08b\1a\86\c0hU\a1]i\b2\89<\12$qE}\10\a7\f0\c2\aa\09"
  "\b5\03\1f\ac\cb\16m\cd\96\9c\14\d1\acs\15L\a2\c4&\97~\\c8\80\bc\c3\19\03"
  "Lh\8do\e5:x\1e\cf9}\d0U\1a\10\03_\c2p\cb\9eI\16\e6B\88\9cD\eb \14\c4\f6"
  "\f2L~\06\dc\9b\9fS\aa\c3\15&)\19v\b4/\e0\1d\08\d3\82\87\e8\944\9bos\1f"
  "\c9\d0\1d\ac\12\e5\c3\b1T\11\dd\00\c1%\a8\13\fcD%WW\de4\de\a9U\14A1/\92"
  "\18;\96\ee,\ed\15\c2U\14kY\91\fd\ba\b6\1e\e5\1d\15<\b4M\99\b5\ec\e2\d7"
  "z\de42\13^e\1aK!\a1\ff\e2\a7\db\8d\19\16\c2\fe\17\b6\fe\e0\9di\89\bf\db"
  "\91R\f1\9f\9br\fe\1d1\9f\ac\02\e2\b5W)\9b\d3\f6C\a1\07\bf\12\fe\c6W\83"
  "Z\a3\ad\f3\81\88\f4\94\89\c9n\17\bd\b8-$1\0c\99p\a2\aa1\fa\eb{J\1dv\93"
  "\9c\b6\9e\a7_\86\a5\0a_|s\8dN\12T\b8Cd\86\91\f7\e7N\cdv[\d00\e2\16i\a6"
  "T\fd\e7u\f5\a1\a2\80Tr\04\bd\9a\1c\01\e8T\fe\b0i9\a5e\d0t\c7"\b6\e0\11"
  "\02"\ea=\1d\c4\87\0e\7f\04Ry\ab\e3X\16\82\aad\8d$\b5)\d2\9e\85\a6W\96\1c"
  "\ef\1b\91\ea^\d86\11ZC\83\13\c8\f6\ddqu\116\a5v\8e\84\950\14d\18ztU\ce"
  "\d2\15\83N\14\b2\e5\ba<\19}\9e\98\d1\ea\81G\1b\12\b1L\8f\cf\f4\c5/\0ec"
  "\ff\c22\b1\0c\11V\dd\1fs\03r\b7\bb\d1;\bfs\7f\ddO\15\ac\d4\e7O\84N\a5*"
  "\c6\0a\afP\df\d4\a3\1a\eb\e4\f0\b1\12Q\a7\da\bbfm\92\0be\a6\10&\1em^W%"
  "Q\d1j\c0\08wN\fe\cf\14\b0e\086\adn\a5\85\85\f0\ca\14\e2\fd\03\1a\8e?\c5"
  "A,e\87sS\d6\feL\ad~B\10q\8f6Rw>iP\e8\8b>\a0X\1eS\14N3\c4&\15\8e\83d\e2"
  ".N\c8\ee\e5g\19"@up\9aq\a4\fd\9a\baazj\df\c1\1f\15HI\86\00\c7\86\de\a0"
  "\14}\8c\a2+\d9\13\1a\9a\db\a7\c0x(\16\c9Y\9c/\8bv\cf\18\a1\80\d2\d1\f0"
  "\96\b2[;p\83\fb-T\03\1fd\90#\83V\9eO\19%&2\bd\9c\14b\13~t\ec#\ec\85\a3"
  "_\ae\af~\ec\c3\99:\18\9d\91\e7,gg\8c\f7\99[\9e\e74@I\1e\02\bb\10|\a0\c0"
  "\b7:@\f9\c2\10!\c8\ed\12\c3\e9\14\9b\c8\b0eI\90\b7\f3T):\a9\173$\da\c1"
  "\fa\1c\bf[t\a50\aa\b3\88\93\1d\a0V(\b9\1crW\b9hg^Jp5|\12Hlr\e7\a3N\ad\e7"
  "B\01\f6\\ccB\1b\17Z\07O\e1L\a2\98\a1\93\813t\7f\13\e2\1c\98d\d1\0cpe\ff"
  "D\fc0\a0\a8/L\0d\12\be\bd\05\10\cc>?V;=\c8\92;\9f\90\16.-\07\14\7f\0e\cf"
  "+\8aLzw\0a\c74\1c=|\84l\0fia[\d6o\ac\8af\fc\a0\11L\9b\a5GS\c39\f2\cb\8b"
  "W-\80;\09\16\1f\02\8f\19(4\c8\ee\ben\ad8`\8a\8b\1bSa\f9\0f\99 =U7el#|6"
  "7\11\a8\b9\f7S\bfh\8c*\85~G,\1b\04\85\15\12\a8\f5(\ef\82/u&^Y\f7!E\e6\1a"
  "\0b\89\99y\d5\b1=\09\d8\da\97:5\eb\cf\10N\eb\ff\d7J\1e\8d\0b\8e\d1=\89"
  "\02\e6\03\15"\e6\ff\8d\ddep\8e\f1E\8d+\83\dfD\1a\d5\ef\bfx\aa?\06\f9\b6"
  "K8\fb\b1\0bk\10\ca\eb\ef\16\95\cfG\b7\a4^\06z\9e\ce\85\14\bd\e6\ab\z\c3"
  "\19\e5M\f6\87\18FB\a7\196p\eby,\1a0\af\f0\f9T\cfk\89\08\10CLf\98\b7 \fc"
  "\dal8*\c3\c6\ab\0a\14T\df\7f~\e5(\bb\11\88\c6\f4s\b8V\0d\19*\d7\1f\de\1e"
  "\f3)\16*\f8\f1\90f\acP\1fz\e6\d3J\f37\daM\1a;\97\1a\c0k\92\13\19\e0\88"
  "\1d\f0\c5P\e1\e0\09=!\b0\06w\18\1f\18\eb$l\f7\a4\19YL\8c)\\c8\94\1e\13"
  "\ef\12\97\a3\1a\07\b0\b7\af\f7\999\fd\1c\13\d8\aa\d7|L\e1\08\9c\a5\9bu"
  "\00\88<\e4\17\8e\95\0d\9c\9f\19\0b\03\8f\02\93\00\aaK\dd\1dy}\88\c1\03"
  "\f0\e6a\99\e1[@JO\aa\12\d7\9c\ea\b1\04\ac`\ba\ff\d9r\d0\1c\e3T\17\0dDe"
  "\de\05\d7\f8\a8\7f\90\8f\04\e4\1b*\1d\88J\ff\aac\86\9b\c9O\ba\d9\82nQ:"
  "\12*\1d\bf\95\fcg\02\bc\e3(\90#\ca\e5\c8\16t\e4.\bb\fb\01\03\ab\1c3t\ac"
  "<\1f{\1c\c9N\fdT=\e1\e1\ea\f1\9f\c8\eb\85\f3\cc\11{\a2<\aa\8cY\9ae\ee\c7"
  "\bafg0@\16\1a\cb\cb\d4\ef\ef\00\ff\e9yi@\81<\d0\1b\f0^\ff\e4\f5\95`?2\ec"
  "A\c8\d0%b\11\ac6?^s\bb8\cf>gR\faD\af\ba\15W\04\cf5P\ea\06\83\0e\01\e78"
  "\16[)\1b\b6b\a1!rR\e4\11\a9`\90\e3\ed\d8\f9\10d\bb\09\aa\0eg]V\d3xt\)O"
  "8\15=*\8cT\d2\c0\f4+\08\97\91\b3\f3b\86\1af\9a\d7t\83\f8x\1be\fe:P\d8\fd"
  "\93\10\00\81\0dR\a46Wb\fe\bdIdN\fd\b8\14@\e1\90fM\04\ed\fa}-\\fd\a1<\e7"
  "\19\c8\8c\1a`\b0"\d4\bcn\9cY>\e5\850\10\fa/!x\+\09l\8a\03\f0\8d^\a7<\14"
  "\f8{)\963v\0b\07m\04l16\d1K\19\f6\da\b3{\c0S\ceH\88\05\c7\bd\83\c5\9e\1f"
  "\dahPMX\f4\80-uc\9cVr;\c3\13\10\83\a4`n1\e1xR|C\ecN\0a\b4\180001020304"
  "0506070809101112131415161718192021222324252627282930313233343536373839"
  "4041424344454647484950515253545556575859606162636465666768697071727374"
  "75767778798081828384858687888990919293949596979899struct variant\00\00"
  "\10H\10\00\0e\00\00\00tuple variant\00\00\00(H\10\00\0d\00\00\00newtyp"
  "e variant\00@H\10\00\0f\00\00\00unit variantXH\10\00\0c\00\00\00enumlH"
  "\10\00\04\00\00\00map\00xH\10\00\03\00\00\00sequence\84H\10\00\08\00\00"
  "\00newtype struct\00\00\94H\10\00\0e\00\00\00Option value\acH\10\00\0c"
  "\00\00\00unit value\00\00\c0H\10\00\0a\00\00\00byte array\00\00\d4H\10"
  "\00\0a\00\00\00string \00\e8H\10\00\07\00\00\00character ``\f8H\10\00\0b"
  "\00\00\00\03I\10\00\01\00\00\00floating point `\14I\10\00\10\00\00\00\03"
  "I\10\00\01\00\00\00integer `\00\00\004I\10\00\09\00\00\00\03I\10\00\01"
  "\00\00\00boolean `\00\00\00PI\10\00\09\00\00\00\03I\10\00\01\00\00\00a"
  " stringa borrowed string\00\00\005\00\00\00\04\00\00\00\04\00\00\006\00"
  "\00\007\00\00\008\00\00\00called `Option::unwrap()` on a `None` value\00"
  "\a0I\10\00\00\00\00\00uncategorized errorother errorout of memoryunexp"
  "ected end of fileunsupportedoperation interruptedargument list too lon"
  "ginvalid filenametoo many linkscross-device link or renamedeadlockexec"
  "utable file busyresource busyfile too largefilesystem quota exceededse"
  "ek on unseekable fileno storage spacewrite zerotimed outinvalid datain"
  "valid input parameterstale network file handlefilesystem loop or indir"
  "ection limit (e.g. symlink loop)read-only filesystem or storage medium"
  "directory not emptyis a directorynot a directoryoperation would blocke"
  "ntity already existsbroken pipenetwork downaddress not availableaddres"
  "s in usenot connectedconnection abortednetwork unreachablehost unreach"
  "ableconnection resetconnection refusedpermission deniedentity not foun"
  "d (os error )\00\00\00\a0I\10\00\00\00\00\00\c1L\10\00\0b\00\00\00\ccL"
  "\10\00\01\00\00\00memory allocation of  bytes failed\0a\00\e8L\10\00\15"
  "\00\00\00\fdL\10\00\0e\00\00\00library/std/src/alloc.rs\1cM\10\00\18\00"
  "\00\00I\01\00\00\09\00\00\00library/std/src/panicking.rsDM\10\00\1c\00"
  "\00\00F\02\00\00\1f\00\00\00DM\10\00\1c\00\00\00G\02\00\00\1e\00\00\00"
  "9\00\00\00\0c\00\00\00\04\00\00\00:\00\00\005\00\00\00\08\00\00\00\04\00"
  "\00\00;\00\00\00<\00\00\00\10\00\00\00\04\00\00\00=\00\00\00>\00\00\00"
  "5\00\00\00\08\00\00\00\04\00\00\00?\00\00\00@\00\00\00operation succes"
  "sful\0e\00\00\00\10\00\00\00\16\00\00\00\15\00\00\00\0b\00\00\00\16\00"
  "\00\00\0d\00\00\00\0b\00\00\00\13\00\00\00\10\00\00\00\10\00\00\00\10\00"
  "\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00"
  "\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00"
  "\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00\00\00\10\00"
  "\00\00\10\00\00\00\10\00\00\00\10\00\00\00\11\00\00\00\12\00\00\00\10\00"
  "\00\00\10\00\00\00\13\00\00\00\12\00\00\00\0d\00\00\00\0e\00\00\00\15\00"
  "\00\00\0c\00\00\00\0b\00\00\00\15\00\00\00\15\00\00\00\0f\00\00\00\0e\00"
  "\00\00\13\00\00\00&\00\00\008\00\00\00\19\00\00\00\17\00\00\00\0c\00\00"
  "\00\09\00\00\00\0a\00\00\00\10\00\00\00\17\00\00\00\19\00\00\00\0e\00\00"
  "\00\0d\00\00\00\14\00\00\00\08\00\00\00\1b\00\00\00[J\10\00KJ\10\005J\10"
  "\00 J\10\00\15J\10\00\ffI\10\00\f2I\10\00\e7I\10\00\d4I\10\00\b1L\10\00"
  "\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00"
  "\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00"
  "\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00\b1L\10\00"
  "\b1L\10\00\b1L\10\00\a0L\10\00\8eL\10\00~L\10\00nL\10\00[L\10\00IL\10\00"
  "<L\10\00.L\10\00\19L\10\00\0dL\10\00\02L\10\00\edK\10\00\d8K\10\00\c9K"
  "\10\00\bbK\10\00\a8K\10\00\82K\10\00JK\10\001K\10\00\1aK\10\00\0eK\10\00"
  "\05K\10\00\fbJ\10\00\ebJ\10\00\d4J\10\00\bbJ\10\00\adJ\10\00\a0J\10\00"
  "\8cJ\10\00\84J\10\00iJ\10\00Hash table capacity overflow\dcO\10\00\1c\00"
  "\00\00/cargo/registry/src/github.com-1ecc6299db9ec823/hashbrown-0.12.0"
  "/src/raw/mod.rs\00\00P\10\00O\00\00\00`\00\00\00(\00\00\00A\00\00\00\04"
  "\00\00\00\04\00\00\00B\00\00\00library/alloc/src/raw_vec.rscapacity ov"
  "erflow\00\00\00\8cP\10\00\11\00\00\00pP\10\00\1c\00\00\00\05\02\00\00\05"
  "\00\00\00FromUtf8Errorbytes\00\00A\00\00\00\04\00\00\00\04\00\00\00C\00"
  "\00\00error\00\00\00A\00\00\00\04\00\00\00\04\00\00\00D\00\00\00assert"
  "ion failed: edelta >= 0library/core/src/num/diy_float.rs\00\00\11Q\10\00"
  "!\00\00\00L\00\00\00\09\00\00\00\11Q\10\00!\00\00\00N\00\00\00\09\00\00"
  "\00\01\00\00\00\0a\00\00\00d\00\00\00\e8\03\00\00\10'\00\00\a0\86\01\00"
  "@B\0f\00\80\96\98\00\00\e1\f5\05\00\ca\9a;\02\00\00\00\14\00\00\00\c8\00"
  "\00\00\d0\07\00\00 N\00\00@\0d\03\00\80\84\1e\00\00-1\01\00\c2\eb\0b\00"
  "\945w\00\00\c1o\f2\86#\00\00\00\00\00\81\ef\ac\85[Am-\ee\04\00\00\00\00"
  "\00\00\00\00\00\00\01\1fj\bfd\ed8n\ed\97\a7\da\f4\f9?\e9\03O\18\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01>\95.\09\99\df\03\fd8\15\0f"
  "/\e4t#\ec\f5\cf\d3\08\dc\04\c4\da\b0\cd\bc\19\7f3\a6\03&\1f\e9N\02\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\01|.\98[\87\d3\ber\9f\d9\d8\87/\15\12\c6P\dek"
  "pnJ\cf\0f\d8\95\d5nq\b2&\b0f\c6\ad$6\15\1dZ\d3B<\0eT\ffc\c0sU\cc\17\ef"
  "\f9e\f2(\bcU\f7\c7\dc\80\dc\edn\f4\ce\ef\dc_\f7S\05\00library/core/src"
  "/num/flt2dec/strategy/dragon.rsassertion failed: d.mant > 0\00|R\10\00"
  "/\00\00\00u\00\00\00\05\00\00\00assertion failed: d.minus > 0\00\00\00"
  "|R\10\00/\00\00\00v\00\00\00\05\00\00\00assertion failed: d.plus > 0|R"
  "\10\00/\00\00\00w\00\00\00\05\00\00\00assertion failed: d.mant.checked"
  "_add(d.plus).is_some()\00\00|R\10\00/\00\00\00x\00\00\00\05\00\00\00as"
  "sertion failed: d.mant.checked_sub(d.minus).is_some()\00|R\10\00/\00\00"
  "\00y\00\00\00\05\00\00\00assertion failed: buf.len() >= MAX_SIG_DIGITS"
  "\00\00\00|R\10\00/\00\00\00z\00\00\00\05\00\00\00|R\10\00/\00\00\00\c1"
  "\00\00\00\09\00\00\00|R\10\00/\00\00\00\fa\00\00\00\0d\00\00\00|R\10\00"
  "/\00\00\00\0a\01\00\00\05\00\00\00|R\10\00/\00\00\00\0b\01\00\00\05\00"
  "\00\00|R\10\00/\00\00\00\0c\01\00\00\05\00\00\00|R\10\00/\00\00\00\0d\01"
  "\00\00\05\00\00\00|R\10\00/\00\00\00\0e\01\00\00\05\00\00\00|R\10\00/\00"
  "\00\00e\01\00\00\0d\00\00\00|R\10\00/\00\00\00q\01\00\00&\00\00\00\00\00"
  "\00\00\dfE\1a=\03\cf\1a\e6\c1\fb\cc\fe\00\00\00\00\ca\c6\9a\c7\17\fep\ab"
  "\dc\fb\d4\fe\00\00\00\00O\dc\bc\be\fc\b1w\ff\f6\fb\dc\fe\00\00\00\00\0c"
  "\d6kA\ef\91V\be\11\fc\e4\fe\00\00\00\00<\fc\7f\90\ad\1f\d0\8d,\fc\ec\fe"
  "\00\00\00\00\83\9aU1(\Q\d3F\fc\f4\fe\00\00\00\00\b5\c9\a6\ad\8f\acq\9d"
  "a\fc\fc\fe\00\00\00\00\cb\8b\ee#w"\9c\ea{\fc\04\ff\00\00\00\00mSx@\91I"
  "\cc\ae\96\fc\0c\ff\00\00\00\00W\ce\b6]y\12<\82\b1\fc\14\ff\00\00\00\00"
  "7V\fbM6\94\10\c2\cb\fc\1c\ff\00\00\00\00O\98H8o\ea\96\90\e6\fc$\ff\00\00"
  "\00\00\c7:\82%\cb\85t\d7\00\fd,\ff\00\00\00\00\f4\97\bf\97\cd\cf\86\a0"
  "\1b\fd4\ff\00\00\00\00\e5\ac*\17\98\0a4\ef5\fd<\ff\00\00\00\00\8e\b25*"
  "\fbg8\b2P\fdD\ff\00\00\00\00;?\c6\d2\df\d4\c8\84k\fdL\ff\00\00\00\00\ba"
  "\cd\d3\1a'D\dd\c5\85\fdT\ff\00\00\00\00\96\c9%\bb\ce\9fk\93\a0\fd\\ff\00"
  "\00\00\00\84\a5b}$l\ac\db\ba\fdd\ff\00\00\00\00\f6\da_\0dXf\ab\a3\d5\fd"
  "l\ff\00\00\00\00&\f1\c3\de\93\f8\e2\f3\ef\fdt\ff\00\00\00\00\b8\80\ff\aa"
  "\a8\ad\b5\b5\0a\fe|\ff\00\00\00\00\8bJ|l\05_b\87%\fe\84\ff\00\00\00\00"
  "S0\c14`\ff\bc\c9?\fe\8c\ff\00\00\00\00U&\ba\91\8c\85N\96Z\fe\94\ff\00\00"
  "\00\00\bd~)p$w\f9\dft\fe\9c\ff\00\00\00\00\8f\b8\e5\b8\9f\bd\df\a6\8f\fe"
  "\a4\ff\00\00\00\00\94}t\88\cf_\a9\f8\a9\fe\ac\ff\00\00\00\00\cf\9b\a8\8f"
  "\93pD\b9\c4\fe\b4\ff\00\00\00\00k\15\0f\bf\f8\f0\08\8a\df\fe\bc\ff\00\00"
  "\00\00\b611eU%\b0\cd\f9\fe\c4\ff\00\00\00\00\ac\7f{\d0\c6\e2?\99\14\ff"
  "\cc\ff\00\00\00\00\06;+*\c4\10\\e4.\ff\d4\ff\00\00\00\00\d3\92si\99$$\aa"
  "I\ff\dc\ff\00\00\00\00\0e\ca\00\83\f2\b5\87\fdc\ff\e4\ff\00\00\00\00\eb"
  "\1a\11\92d\08\e5\bc~\ff\ec\ff\00\00\00\00\cc\88Po\09\cc\bc\8c\99\ff\f4"
  "\ff\00\00\00\00,e\19\e2X\17\b7\d1\b3\ff\fc\ff\00\00\00\00\00\00\00\00\00"
  "\00@\9c\ce\ff\04\00\00\00\00\00\00\00\00\00\10\a5\d4\e8\e8\ff\0c\00\00"
  "\00\00\00\00\00b\ac\c5\ebx\ad\03\00\14\00\00\00\00\00\84\09\94\f8x9?\81"
  "\1e\00\1c\00\00\00\00\00\b3\15\07\c9{\ce\97\c08\00$\00\00\00\00\00p\\ea"
  "{\ce2~\8fS\00,\00\00\00\00\00h\80\e9\ab\a48\d2\d5m\004\00\00\00\00\00E"
  ""\9a\17&'O\9f\88\00<\00\00\00\00\00'\fb\c4\d41\a2c\ed\a2\00D\00\00\00\00"
  "\00\a8\ad\c8\8c8e\de\b0\bd\00L\00\00\00\00\00\dbe\ab\1a\8e\08\c7\83\d8"
  "\00T\00\00\00\00\00\9a\1dqB\f9\1d]\c4\f2\00\\00\00\00\00\00X\e7\1b\a6,"
  "iM\92\0d\01d\00\00\00\00\00\ea\8dp\1ad\ee\01\da'\01l\00\00\00\00\00Jw\ef"
  "\9a\99\a3m\a2B\01t\00\00\00\00\00\85k}\b4{x\09\f2\\01|\00\00\00\00\00w"
  "\18\ddy\a1\e4T\b4w\01\84\00\00\00\00\00\c2\c5\9b[\92\86[\86\92\01\8c\00"
  "\00\00\00\00=]\96\c8\c5S5\c8\ac\01\94\00\00\00\00\00\b3\a0\97\fa\\b4*\95"
  "\c7\01\9c\00\00\00\00\00\e3_\a0\99\bd\9fF\de\e1\01\a4\00\00\00\00\00%\8c"
  "9\db4\c2\9b\a5\fc\01\ac\00\00\00\00\00\\9f\98\a3r\9a\c6\f6\16\02\b4\00"
  "\00\00\00\00\ce\be\e9TS\bf\dc\b71\02\bc\00\00\00\00\00\e2A"\f2\17\f3\fc"
  "\88L\02\c4\00\00\00\00\00\a5x\\d3\9b\ce \ccf\02\cc\00\00\00\00\00\dfS!"
  "{\f3Z\16\98\81\02\d4\00\00\00\00\00:0\1f\97\dc\b5\a0\e2\9b\02\dc\00\00"
  "\00\00\00\96\b3\e3\S\d1\d9\a8\b6\02\e4\00\00\00\00\00<D\a7\a4\d9|\9b\fb"
  "\d0\02\ec\00\00\00\00\00\10D\a4\a7LLv\bb\eb\02\f4\00\00\00\00\00\1a\9c"
  "@\b6\ef\8e\ab\8b\06\03\fc\00\00\00\00\00,\84W\a6\10\ef\1f\d0 \03\04\01"
  "\00\00\00\00)1\91\e9\e5\a4\10\9b;\03\0c\01\00\00\00\00\9d\0c\9c\a1\fb\9b"
  "\10\e7U\03\14\01\00\00\00\00)\f4;b\d9 (\acp\03\1c\01\00\00\00\00\85\cf"
  "\a7z^KD\80\8b\03$\01\00\00\00\00-\dd\ac\03@\e4!\bf\a5\03,\01\00\00\00\00"
  "\8f\ffD^/\9cg\8e\c0\034\01\00\00\00\00A\b8\8c\9c\9d\173\d4\da\03<\01\00"
  "\00\00\00\a9\1b\e3\b4\92\db\19\9e\f5\03D\01\00\00\00\00\d9w\df\ban\bf\96"
  "\eb\0f\04L\01\00\00\00\00library/core/src/num/flt2dec/strategy/grisu.r"
  "s\00\00\a8Y\10\00.\00\00\00}\00\00\00\15\00\00\00\a8Y\10\00.\00\00\00\a9"
  "\00\00\00\05\00\00\00\a8Y\10\00.\00\00\00\aa\00\00\00\05\00\00\00\a8Y\10"
  "\00.\00\00\00\ab\00\00\00\05\00\00\00\a8Y\10\00.\00\00\00\ac\00\00\00\05"
  "\00\00\00\a8Y\10\00.\00\00\00\ad\00\00\00\05\00\00\00\a8Y\10\00.\00\00"
  "\00\ae\00\00\00\05\00\00\00assertion failed: d.mant + d.plus < (1 << 6"
  "1)\00\00\00\a8Y\10\00.\00\00\00\af\00\00\00\05\00\00\00\a8Y\10\00.\00\00"
  "\00\0b\01\00\00\11\00\00\00\00\00\00\00\00\00\00\00attempt to divide b"
  "y zero\00\00\00\a8Y\10\00.\00\00\00\0e\01\00\00\09\00\00\00\a8Y\10\00."
  "\00\00\00C\01\00\00\09\00\00\00assertion failed: !buf.is_empty()\00\00"
  "\00\a8Y\10\00.\00\00\00\e0\01\00\00\05\00\00\00assertion failed: d.man"
  "t < (1 << 61)\a8Y\10\00.\00\00\00\e1\01\00\00\05\00\00\00\a8Y\10\00.\00"
  "\00\00\e2\01\00\00\05\00\00\00\a8Y\10\00.\00\00\00'\02\00\00\11\00\00\00"
  "\a8Y\10\00.\00\00\00*\02\00\00\09\00\00\00\a8Y\10\00.\00\00\00`\02\00\00"
  "\09\00\00\00library/core/src/num/flt2dec/mod.rs\00\84[\10\00#\00\00\00"
  "\bc\00\00\00\05\00\00\00assertion failed: buf[0] > b\'0\'\00\00\00\84["
  "\10\00#\00\00\00\bd\00\00\00\05\00\00\00assertion failed: parts.len() "
  ">= 4\00\00\84[\10\00#\00\00\00\be\00\00\00\05\00\00\000..-+\00\00\000i"
  "nfNaNassertion failed: buf.len() >= maxlen\84[\10\00#\00\00\00\7f\02\00"
  "\00\0d\00\00\00..\00\00d\\10\00\02\00\00\00called `Option::unwrap()` o"
  "n a `None` value\00L\00\00\00\00\00\00\00\01\00\00\00M\00\00\00(\\10\00"
  "\00\00\00\00index out of bounds: the len is  but the index is \00\00\b4"
  "\\10\00 \00\00\00\d4\\10\00\12\00\00\00L\00\00\00\04\00\00\00\04\00\00"
  "\00N\00\00\00L\00\00\00\04\00\00\00\04\00\00\00O\00\00\00matches!===as"
  "sertion failed: `(left  right)`\0a  left: ``,\0a right: ``: \00\00\00#"
  "]\10\00\19\00\00\00<]\10\00\12\00\00\00N]\10\00\0c\00\00\00Z]\10\00\03"
  "\00\00\00`\00\00\00#]\10\00\19\00\00\00<]\10\00\12\00\00\00N]\10\00\0c"
  "\00\00\00\80]\10\00\01\00\00\00: \00\00(\\10\00\00\00\00\00\a4]\10\00\02"
  "\00\00\00L\00\00\00\0c\00\00\00\04\00\00\00P\00\00\00Q\00\00\00R\00\00"
  "\00     {\0a,\0a,  { } }(\0a(,)\0a[L\00\00\00\04\00\00\00\04\00\00\00S"
  "\00\00\00]0x0001020304050607080910111213141516171819202122232425262728"
  "2930313233343536373839404142434445464748495051525354555657585960616263"
  "6465666768697071727374757677787980818283848586878889909192939495969798"
  "99\00L\00\00\00\04\00\00\00\04\00\00\00T\00\00\00U\00\00\00V\00\00\000"
  "000000000000000000000000000000000000000000000000000000000000000truefal"
  "serange start index  out of range for slice of length \00\00\00%_\10\00"
  "\12\00\00\007_\10\00"\00\00\00library/core/src/slice/index.rs\00l_\10\00"
  "\1f\00\00\004\00\00\00\05\00\00\00range end index \9c_\10\00\10\00\00\00"
  "7_\10\00"\00\00\00l_\10\00\1f\00\00\00I\00\00\00\05\00\00\00slice inde"
  "x starts at  but ends at \00\cc_\10\00\16\00\00\00\e2_\10\00\0d\00\00\00"
  "l_\10\00\1f\00\00\00\\00\00\00\05\00\00\00incomplete utf-8 byte sequen"
  "ce from index \00\00\10`\10\00*\00\00\00invalid utf-8 sequence of  byt"
  "es from index D`\10\00\1a\00\00\00^`\10\00\12\00\00\00\01\01\01\01\01\01"
  "\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
  "\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
  "\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
  "\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
  "\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01"
  "\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"
  "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02"
  "\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02"
  "\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00"
  "\00\00\00\00\00\00\00\00\00\00library/core/src/str/pattern.rs\00\80a\10"
  "\00\1f\00\00\00\1a\06\00\00\15\00\00\00\80a\10\00\1f\00\00\00H\06\00\00"
  "\15\00\00\00\80a\10\00\1f\00\00\00I\06\00\00\15\00\00\00library/core/s"
  "rc/str/mod.rs[...]byte index  is out of bounds of `\00\00\00\f0a\10\00"
  "\0b\00\00\00\fba\10\00\16\00\00\00\80]\10\00\01\00\00\00\d0a\10\00\1b\00"
  "\00\00k\00\00\00\09\00\00\00begin <= end ( <= ) when slicing `\00\00<b"
  "\10\00\0e\00\00\00Jb\10\00\04\00\00\00Nb\10\00\10\00\00\00\80]\10\00\01"
  "\00\00\00\d0a\10\00\1b\00\00\00o\00\00\00\05\00\00\00\d0a\10\00\1b\00\00"
  "\00}\00\00\00-\00\00\00 is not a char boundary; it is inside  (bytes )"
  " of `\f0a\10\00\0b\00\00\00\a0b\10\00&\00\00\00\c6b\10\00\08\00\00\00\ce"
  "b\10\00\06\00\00\00\80]\10\00\01\00\00\00\d0a\10\00\1b\00\00\00\7f\00\00"
  "\00\05\00\00\00library/core/src/unicode/printable.rs\00\00\00\0cc\10\00"
  "%\00\00\00\1a\00\00\006\00\00\00\00\01\03\05\05\06\06\02\07\06\08\07\09"
  "\11\0a\1c\0b\19\0c\1a\0d\10\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\04\18"
  "\01\19\03\1a\07\1b\01\1c\02\1f\16 \03+\03-\0b.\010\031\022\01\a7\02\a9"
  "\02\aa\04\ab\08\fa\02\fb\05\fd\02\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90"
  "\1c\dd\0e\0fKL\fb\fc./?\]_\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de"
  "\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4"
  "\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11):;EIW[\^_"
  "de\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7"
  "\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^"
  "_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\\f6\f7\fe\ff\80mq\de\df\0e"
  "\1fno\1c\1d_}~\ae\af\7f\bb\bc\16\17\1e\1fFGNOXZ\^~\7f\b5\c5\d4\d5\dc\f0"
  "\f1\f5rs\8ftu\96&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\d2\d4\ce\ff"
  "NOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91Sgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00"
  " _"\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab\05\1f\09\81\1b\03\19\08"
  "\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03"
  "\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05N\07\1b\07W\07\02"
  "\06\16\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0"
  "\03\1a\06\82\fd\03Y\07\16\09\18\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05"
  "F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06/1M\03\80\a4\08"
  "<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03!\0f!\0f\80\8c\04\82\97\19"
  "\0b\15\88\94\05/\05;\07\02\0e\18\09\80\be"t\0c\80\d6\1a\0c\05\80\ff\05"
  "\80\df\0c\f2\9d\037\09\81\\14\80\b8\08\80\cb\05\0a\18;\03\0a\068\08F\08"
  "\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17"
  "\041\a1\04\81\da&\07\0c\05\05\80\a6\10\81\f5\07\01 *\06L\04\80\8d\04\80"
  "\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\05\07\07\02\08\08\09\02\0a\05"
  "\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08"
  "$\01j\04k\02\af\03\bc\02\cf\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05"
  "\e1\02\e7\04\e8\02\ee \f0\04\f8\02\fa\02\fb\01\0c';>NO\8f\9e\9e\9f{\8b"
  "\93\96\a2\b2\ba\86\b1\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd"
  "5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\\b6\b7\1b\1c\07\08\0a\0b"
  "\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\bf\ee\efZb\f4\fc\ff"
  "\9a\9b./'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc"
  "\cd\a0\07\19\1a"%>?\e7\ec\ef\ff\c5\c6\04 #%&(38:HJLPSUVXZ\^`cefksx}\7f"
  "\8a\a4\aa\af\b0\c0\d0\ae\afno\93^"{\05\03\04-\03f\03\01/.\80\82\1d\031"
  "\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0bNC\817\09\16\0a"
  "\08\18;E9\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04"
  "\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RN(\08*\16\1a&\1c\14"
  "\17\09N\04$\09D\0d\19\07\0a\06H\08'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10"
  "\03\05\80\8bb\1eH\08\0a\80\a6^"E\0b\0a\06\0d\13:\06\0a6,\04\17\80\b9<d"
  "S\0cH\09\0aFE\1bH\08S\0dI\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816"
  "\19\80\b7\01\0f2\0d\83\9bfu\0b\80\c4\8aLc\0d\84/\8f\d1\82G\a1\b9\829\07"
  "*\04\\06&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6"
  "*\09\a2\e7\813-\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\92`G\09t<\80"
  "\f6\0as\08p\15F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\84P\1f\80\e1"
  "+\80\d5-\03\1a\04\02\81@\1f\11:\05\01\84\e0\80\f7)L\04\0a\04\02\83\11D"
  "L=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07"
  "\02\0e\06\80\9a\83\d8\05\10\03\0d\03t\0cY\07\0c\04\01\0f\0c\048\08\0a\06"
  "(\08"N\81T\0c\15\03\05\03\07\09\1d\03\0b\05\06\0a\0a\06\08\08\07\09\80"
  "\cb%\0a\84\06library/core/src/unicode/unicode_data.rs\00\00\00\adh\10\00"
  "(\00\00\00K\00\00\00(\00\00\00\adh\10\00(\00\00\00W\00\00\00\16\00\00\00"
  "\adh\10\00(\00\00\00R\00\00\00>\00\00\00library/core/src/num/bignum.rs"
  "\00\00\08i\10\00\1e\00\00\00\ac\01\00\00\01\00\00\00assertion failed: "
  "noborrowassertion failed: digits < 40assertion failed: other > 0SomeNo"
  "neErrorUtf8Errorvalid_up_toerror_lenL\00\00\00\04\00\00\00\04\00\00\00"
  "W\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17 \1f\0c `\1f"
  "\ef,\a0+*0 ,o\a6\e0,\02\a8`-\1e\fb`.\00\fe 6\9e\ff`6\fd\01\e16\01\0a!7"
  "$\0d\e17\ab\0ea9/\18\a190\1c\e1G\f3\1e!L\f0j\e1OOo!P\9d\bc\a1P\00\cfaQ"
  "e\d1\a1Q\00\da!R\00\e0\e1S0\e1aU\ae\e2\a1V\d0\e8\e1V \00nW\f0\01\ffW\00"
  "p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04"
  "#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03<\08*\18\01 7\01\01"
  "\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a"
  "\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02"
  "\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01"
  "\01\0c\01\09\01(\01\03\017\01\01\03\05\03\01\04\07\02\0b\02\1d\01:\01\02"
  "\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a"
  "\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\06J"
  "\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02"
  "\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\02\1e\02\1e"
  "\02@\02\01\07\08\01\02\0b\09\01-\03\01\01u\02"\01v\03\04\02\09\01\06\03"
  "\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\1f1\040\07\01\01"
  "\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03"
  "\01\0d\01\07\04\01\06\01\03\02\c6@\00\01\c3!\00\03\8d\01` \00\06i\02\00"
  "\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19"
  "\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02"
  "\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00"
  "\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02\99\0b1\04"
  "{\016\0f)\01\02\02\0a\031\04\02\02\07\01=\03$\05\01\08>\01\0c\024\09\0a"
  "\04\02\01_\03\02\01\01\02\06\01\a0\01\03\08\15\029\02\01\01\01\01\16\01"
  "\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02"
  "\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03"
  "\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a"
  "(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02"
  "\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\00\05;\07\00\01"
  "?\04Q\01\00\02\00.\02\17\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007"
  "\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05\00\07\00\01"
  "=\04\00\07m\07\00`\80\f0\00";

function f_a(a:{ a:int, b:int, c:short }, b:int, c:byte_ptr, d:int) {
  var g:int_ptr;
  var i:int;
  var k:int;
  var l:int;
  var m:int;
  var r:int;
  var o:int;
  var s:int;
  var t:int;
  var p:byte_ptr;
  var n:int;
  var j:int;
  var qa:long;
  var v:int;
  var u:int;
  var q:int;
  var aa:int;
  var oa:int;
  var f:int;
  var w:int;
  var na:int;
  var ma:int;
  var h:int;
  var e:int = g_a - 1344;
  g_a = e;
  qa = b[0]:long;
  if (eqz(qa)) goto B_u;
  var ra:long = b[1]:long;
  if (eqz(ra)) goto B_t;
  var sa:long = b[2]:long;
  if (eqz(sa)) goto B_s;
  var ta:long = qa + sa;
  if (ta < qa) goto B_r;
  if (qa - ra > qa) goto B_q;
  if (d < 17) goto B_p;
  f = b[26]:byte;
  b = b[12]:ushort;
  e[1]:int = qa;
  (e + 8)[0]:int =
    select_if(0, i32_wrap_i64(qa >> 32L), g = qa < 4294967296L);
  e[0]:int = select_if(1, 2, g);
  f_bk(e + 12, 0, 152);
  e[43]:int = ra;
  (e + 168 + 8)[0]:int =
    select_if(0, i32_wrap_i64(ra >> 32L), g = ra < 4294967296L);
  e[42]:int = select_if(1, 2, g);
  f_bk(e + 168 + 12, 0, 152);
  e[85]:int = sa;
  (e + 336 + 8)[0]:int =
    select_if(0, i32_wrap_i64(sa >> 32L), g = sa < 4294967296L);
  e[84]:int = select_if(1, 2, g);
  f_bk(e + 336 + 12, 0, 152);
  f_bk(e + 504 + 8, 0, 156);
  e[63]:long = 4294967297L;
  g = 
    i32_wrap_i64(
      (((i64_extend_i32_u(b) << 48L) >> 48L) - clz(ta + -1L)) * 1292913986L +
      82746495104L >>
      32L);
  h = (g << 16) >> 16;
  i = (b << 16) >> 16;
  if (i < 0) goto B_w;
  f_q(e, b);
  f_q(e + 168, b);
  f_q(e + 336, b);
  goto B_v;
  label B_w:
  f_q(e + 504, (0 - i << 16) >> 16);
  label B_v:
  if (h > -1) goto B_y;
  f_d(e, b = (0 - h << 16) >> 16);
  f_d(e + 168, b);
  f_d(e + 336, b);
  goto B_x;
  label B_y:
  f_d(e + 504, g & 65535);
  label B_x:
  j = e[0]:int;
  f_dk(e + 1176 | 4, k = e | 4, 160);
  e[294]:int = j;
  m = select_if(j, l = e[84]:int, j > l);
  if (m > 40) goto B_ca;
  if (m) goto B_da;
  m = 0;
  goto B_z;
  label B_da:
  n = m & 1;
  if (m != 1) goto B_ba;
  o = 0;
  p = 0;
  goto B_aa;
  label B_ca:
  f_sj(m, 40, e);
  unreachable;
  label B_ba:
  q = m & -2;
  g = e + 336 + 8;
  b = e + 1176 + 8;
  o = 0;
  p = 0;
  loop L_ea {
    i = b + -4;
    i[0]:int = (s = (i = (r = i[0]:int) + (g + -4)[0]:int) + (o & 1));
    b[0]:int = (i = (o = (t = b[0]:int) + g[0]) + (i < r | s < i));
    o = o < t | i < o;
    g = g + 8;
    b = b + 8;
    if (q != (p = p + 2)) continue L_ea;
  }
  label B_aa:
  if (eqz(n)) goto B_fa;
  g = e + 1176 + (b = p << 2) + 4;
  g[0] = (i = (b = (g = g[0]) + (e + 336 + b + 4)[0]:int) + o);
  o = b < g | i < b;
  label B_fa:
  if (eqz(o & 1)) goto B_z;
  if (m > 39) goto B_o;
  (e + 1176 + (m << 2) + 4)[0]:int = 1;
  m = m + 1;
  label B_z:
  e[294]:int = m;
  p = e[126]:int;
  b = select_if(p, m, p > m);
  if (b >= 41) goto B_n;
  u = e + 336 | 4;
  v = e + 168 | 4;
  w = e | 4;
  b = b << 2;
  loop L_ha {
    if (b) goto B_ia;
    g = select_if(-1, 0, b);
    goto B_ga;
    label B_ia:
    g = e + 1176 + b;
    i = e + 504 + b;
    b = b + -4;
    g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
    if (eqz(g)) continue L_ha;
  }
  label B_ga:
  if (g < f) goto B_ja;
  if (j >= 41) goto B_m;
  if (j) goto B_ka;
  j = 0;
  goto B_k;
  label B_ka:
  b = j + -1 & 1073741823;
  i = b + 1;
  g = i & 3;
  if (b >= 3) goto B_la;
  qa = 0L;
  b = w;
  goto B_l;
  label B_la:
  i = i & 2147483644;
  qa = 0L;
  b = w;
  loop L_ma {
    b[0]:int = (qa = b[0]:uint * 10L + qa);
    o = b + 4;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    o = b + 8;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    o = b + 12;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    qa = qa >> 32L;
    b = b + 16;
    i = i + -4;
    if (i) continue L_ma;
    goto B_l;
  }
  label B_ja:
  h = h + 1;
  goto B_c;
  label B_u:
  f_rf(1069739, 28, 1069768);
  unreachable;
  label B_t:
  f_rf(1069784, 29, 1069816);
  unreachable;
  label B_s:
  f_rf(1069832, 28, 1069860);
  unreachable;
  label B_r:
  f_rf(1069876, 54, 1069932);
  unreachable;
  label B_q:
  f_rf(1069948, 55, 1070004);
  unreachable;
  label B_p:
  f_rf(1070020, 45, 1070068);
  unreachable;
  label B_o:
  f_ne(m, 40, 1075496);
  unreachable;
  label B_n:
  f_sj(b, 40, e);
  unreachable;
  label B_m:
  f_sj(j, 40, e);
  unreachable;
  label B_l:
  if (eqz(g)) goto B_na;
  loop L_oa {
    b[0]:int = (qa = b[0]:uint * 10L + qa);
    b = b + 4;
    qa = qa >> 32L;
    g = g + -1;
    if (g) continue L_oa;
  }
  label B_na:
  b = i32_wrap_i64(qa);
  if (eqz(b)) goto B_k;
  if (j > 39) goto B_j;
  (e + (j << 2) + 4)[0]:int = b;
  j = j + 1;
  label B_k:
  e[0]:int = j;
  r = e[42]:int;
  if (r >= 41) goto B_i;
  if (r) goto B_pa;
  r = 0;
  goto B_g;
  label B_pa:
  b = r + -1 & 1073741823;
  i = b + 1;
  g = i & 3;
  if (b >= 3) goto B_qa;
  qa = 0L;
  b = v;
  goto B_h;
  label B_qa:
  i = i & 2147483644;
  qa = 0L;
  b = v;
  loop L_ra {
    b[0]:int = (qa = b[0]:uint * 10L + qa);
    o = b + 4;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    o = b + 8;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    o = b + 12;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    qa = qa >> 32L;
    b = b + 16;
    i = i + -4;
    if (i) continue L_ra;
    goto B_h;
  }
  label B_j:
  f_ne(j, 40, 1075496);
  unreachable;
  label B_i:
  f_sj(r, 40, e);
  unreachable;
  label B_h:
  if (eqz(g)) goto B_sa;
  loop L_ta {
    b[0]:int = (qa = b[0]:uint * 10L + qa);
    b = b + 4;
    qa = qa >> 32L;
    g = g + -1;
    if (g) continue L_ta;
  }
  label B_sa:
  b = i32_wrap_i64(qa);
  if (eqz(b)) goto B_g;
  if (r > 39) goto B_f;
  (e + 168 + (r << 2) + 4)[0]:int = b;
  r = r + 1;
  label B_g:
  e[42]:int = r;
  if (l >= 41) goto B_e;
  if (l) goto B_ua;
  e[84]:int = 0;
  goto B_c;
  label B_ua:
  b = l + -1 & 1073741823;
  i = b + 1;
  g = i & 3;
  if (b >= 3) goto B_va;
  qa = 0L;
  b = u;
  goto B_d;
  label B_va:
  i = i & 2147483644;
  qa = 0L;
  b = u;
  loop L_wa {
    b[0]:int = (qa = b[0]:uint * 10L + qa);
    o = b + 4;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    o = b + 8;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    o = b + 12;
    o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
    qa = qa >> 32L;
    b = b + 16;
    i = i + -4;
    if (i) continue L_wa;
    goto B_d;
  }
  label B_f:
  f_ne(r, 40, 1075496);
  unreachable;
  label B_e:
  f_sj(l, 40, e);
  unreachable;
  label B_d:
  if (eqz(g)) goto B_xa;
  loop L_ya {
    b[0]:int = (qa = b[0]:uint * 10L + qa);
    b = b + 4;
    qa = qa >> 32L;
    g = g + -1;
    if (g) continue L_ya;
  }
  label B_xa:
  b = i32_wrap_i64(qa);
  if (eqz(b)) goto B_za;
  if (l > 39) goto B_b;
  (e + 336 + (l << 2) + 4)[0]:int = b;
  l = l + 1;
  label B_za:
  e[84]:int = l;
  label B_c:
  f_dk(e + 672 | 4, b = e + 504 | 4, 160);
  e[168]:int = p;
  var x:int = f_q(e + 672, 1);
  g = e[126]:int;
  f_dk(e + 840 | 4, b, 160);
  e[210]:int = g;
  var y:int = f_q(e + 840, 2);
  g = e[126]:int;
  f_dk(e + 1008 | 4, b, 160);
  e[252]:int = g;
  var z:int = f_q(e + 1008, 3);
  q = e[0]:int;
  m = select_if(q, aa = e[252]:int, q > aa);
  if (m > 40) goto B_qb;
  var ba:int = e + 336 + 8;
  var ca:int = e + 1176 + 8;
  var da:int = e + 504 + 8;
  var ea:int = e + 672 + 8;
  var fa:int = e + 840 + 8;
  var ga:int = e + 1008 + 8;
  var ha:int = e + 8;
  var ia:int = e + 1176 | 4;
  var ja:int = e[126]:int;
  var ka:int = e[168]:int;
  var la:int = e[210]:int;
  ma = 0;
  loop L_rb {
    na = ma;
    b = m << 2;
    loop L_tb {
      if (b) goto B_ub;
      g = select_if(-1, 0, b);
      goto B_sb;
      label B_ub:
      g = e + 1008 + b;
      i = e + b;
      b = b + -4;
      g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
      if (eqz(g)) continue L_tb;
    }
    label B_sb:
    l = 0;
    if (g >= 2) goto B_vb;
    if (eqz(m)) goto B_xb;
    o = 1;
    j = m & 1;
    p = 0;
    if (m == 1) goto B_yb;
    q = m & -2;
    p = 0;
    o = 1;
    g = ga;
    b = ha;
    loop L_zb {
      i = b + -4;
      i[0]:int =
        (s = (i = (r = i[0]:int) + ((g + -4)[0]:int ^ -1)) + (o & 1));
      b[0]:int = (i = (o = (t = b[0]:int) + (g[0] ^ -1)) + (i < r | s < i));
      o = o < t | i < o;
      g = g + 8;
      b = b + 8;
      if (q != (p = p + 2)) continue L_zb;
    }
    label B_yb:
    if (eqz(j)) goto B_ac;
    g = e + (b = p << 2) + 4;
    g[0] = (i = (b = (g = g[0]) + ((z + b + 4)[0]:int ^ -1)) + o);
    o = b < g | i < b;
    label B_ac:
    if (eqz(o & 1)) goto B_wb;
    label B_xb:
    e[0]:int = m;
    l = 8;
    q = m;
    goto B_vb;
    label B_wb:
    f_rf(1075512, 26, 1075496);
    unreachable;
    label B_vb:
    m = select_if(q, la, q > la);
    if (m >= 41) goto B_dc;
    b = m << 2;
    loop L_fc {
      if (b) goto B_gc;
      g = select_if(-1, 0, b);
      goto B_ec;
      label B_gc:
      g = e + 840 + b;
      i = e + b;
      b = b + -4;
      g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
      if (eqz(g)) continue L_fc;
    }
    label B_ec:
    if (g < 2) goto B_hc;
    m = q;
    goto B_bc;
    label B_hc:
    if (eqz(m)) goto B_ic;
    o = 1;
    j = m & 1;
    p = 0;
    if (m == 1) goto B_jc;
    q = m & -2;
    p = 0;
    o = 1;
    g = fa;
    b = ha;
    loop L_kc {
      i = b + -4;
      i[0]:int =
        (s = (i = (r = i[0]:int) + ((g + -4)[0]:int ^ -1)) + (o & 1));
      b[0]:int = (i = (o = (t = b[0]:int) + (g[0] ^ -1)) + (i < r | s < i));
      o = o < t | i < o;
      g = g + 8;
      b = b + 8;
      if (q != (p = p + 2)) continue L_kc;
    }
    label B_jc:
    if (eqz(j)) goto B_lc;
    g = e + (b = p << 2) + 4;
    g[0] = (i = (b = (g = g[0]) + ((y + b + 4)[0]:int ^ -1)) + o);
    o = b < g | i < b;
    label B_lc:
    if (eqz(o & 1)) goto B_cc;
    label B_ic:
    e[0]:int = m;
    l = l | 4;
    goto B_bc;
    label B_dc:
    f_sj(m, 40, e);
    unreachable;
    label B_cc:
    f_rf(1075512, 26, 1075496);
    unreachable;
    label B_bc:
    j = select_if(m, ka, m > ka);
    if (j >= 41) goto B_oc;
    b = j << 2;
    loop L_qc {
      if (b) goto B_rc;
      g = select_if(-1, 0, b);
      goto B_pc;
      label B_rc:
      g = e + 672 + b;
      i = e + b;
      b = b + -4;
      g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
      if (eqz(g)) continue L_qc;
    }
    label B_pc:
    if (g < 2) goto B_sc;
    j = m;
    goto B_mc;
    label B_sc:
    if (eqz(j)) goto B_tc;
    o = 1;
    m = j & 1;
    p = 0;
    if (j == 1) goto B_uc;
    q = j & -2;
    p = 0;
    o = 1;
    g = ea;
    b = ha;
    loop L_vc {
      i = b + -4;
      i[0]:int =
        (s = (i = (r = i[0]:int) + ((g + -4)[0]:int ^ -1)) + (o & 1));
      b[0]:int = (i = (o = (t = b[0]:int) + (g[0] ^ -1)) + (i < r | s < i));
      o = o < t | i < o;
      g = g + 8;
      b = b + 8;
      if (q != (p = p + 2)) continue L_vc;
    }
    label B_uc:
    if (eqz(m)) goto B_wc;
    g = e + (b = p << 2) + 4;
    g[0] = (i = (b = (g = g[0]) + ((x + b + 4)[0]:int ^ -1)) + o);
    o = b < g | i < b;
    label B_wc:
    if (eqz(o & 1)) goto B_nc;
    label B_tc:
    e[0]:int = j;
    l = l + 2;
    goto B_mc;
    label B_oc:
    f_sj(j, 40, e);
    unreachable;
    label B_nc:
    f_rf(1075512, 26, 1075496);
    unreachable;
    label B_mc:
    q = select_if(j, ja, j > ja);
    if (q >= 41) goto B_ob;
    b = q << 2;
    loop L_yc {
      if (b) goto B_zc;
      g = select_if(-1, 0, b);
      goto B_xc;
      label B_zc:
      g = e + 504 + b;
      i = e + b;
      b = b + -4;
      g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
      if (eqz(g)) continue L_yc;
    }
    label B_xc:
    if (g < 2) goto B_bd;
    q = j;
    goto B_ad;
    label B_bd:
    if (eqz(q)) goto B_cd;
    o = 1;
    j = q & 1;
    p = 0;
    if (q == 1) goto B_dd;
    m = q & -2;
    p = 0;
    o = 1;
    g = da;
    b = ha;
    loop L_ed {
      i = b + -4;
      i[0]:int =
        (s = (i = (r = i[0]:int) + ((g + -4)[0]:int ^ -1)) + (o & 1));
      b[0]:int = (i = (o = (t = b[0]:int) + (g[0] ^ -1)) + (i < r | s < i));
      o = o < t | i < o;
      g = g + 8;
      b = b + 8;
      if (m != (p = p + 2)) continue L_ed;
    }
    label B_dd:
    if (eqz(j)) goto B_fd;
    g = e + (b = p << 2) + 4;
    g[0] = (i = (b = (g = g[0]) + ((e + 504 + b + 4)[0]:int ^ -1)) + o);
    o = b < g | i < b;
    label B_fd:
    if (eqz(o & 1)) goto B_nb;
    label B_cd:
    e[0]:int = q;
    l = l + 1;
    label B_ad:
    if (na == d) goto B_ib;
    (c + na)[0]:byte = l + 48;
    b = select_if(q, n = e[42]:int, q > n);
    if (b >= 41) goto B_mb;
    ma = na + 1;
    b = b << 2;
    loop L_hd {
      if (b) goto B_id;
      m = select_if(-1, 0, b);
      goto B_gd;
      label B_id:
      g = e + 168 + b;
      i = e + b;
      b = b + -4;
      m = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
      if (eqz(m)) continue L_hd;
    }
    label B_gd:
    f_dk(ia, k, 160);
    e[294]:int = q;
    l = select_if(q, oa = e[84]:int, q > oa);
    if (l > 40) goto B_lb;
    if (l) goto B_kd;
    l = 0;
    goto B_jd;
    label B_kd:
    var pa:int = l & 1;
    o = 0;
    p = 0;
    if (l == 1) goto B_ld;
    j = l & -2;
    o = 0;
    g = ba;
    b = ca;
    p = 0;
    loop L_md {
      i = b + -4;
      i[0]:int = (s = (i = (r = i[0]:int) + (g + -4)[0]:int) + (o & 1));
      b[0]:int = (i = (o = (t = b[0]:int) + g[0]) + (i < r | s < i));
      o = o < t | i < o;
      g = g + 8;
      b = b + 8;
      if (j != (p = p + 2)) continue L_md;
    }
    label B_ld:
    if (eqz(pa)) goto B_nd;
    g = e + 1176 + (b = p << 2) + 4;
    g[0] = (i = (b = (g = g[0]) + (e + 336 + b + 4)[0]:int) + o);
    o = b < g | i < b;
    label B_nd:
    if (eqz(o & 1)) goto B_jd;
    if (l > 39) goto B_kb;
    (e + 1176 + (l << 2) + 4)[0]:int = 1;
    l = l + 1;
    label B_jd:
    e[294]:int = l;
    b = select_if(ja, l, ja > l);
    if (b >= 41) goto B_jb;
    b = b << 2;
    loop L_pd {
      if (b) goto B_qd;
      g = select_if(-1, 0, b);
      goto B_od;
      label B_qd:
      g = e + 1176 + b;
      i = e + 504 + b;
      b = b + -4;
      g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
      if (eqz(g)) continue L_pd;
    }
    label B_od:
    if (m < f) goto B_pb;
    if (g < f) goto B_pb;
    if (q >= 41) goto B_hb;
    if (q) goto B_sd;
    q = 0;
    goto B_rd;
    label B_sd:
    i = q + -1 & 1073741823;
    o = i + 1;
    g = o & 3;
    qa = 0L;
    b = w;
    if (i < 3) goto B_td;
    i = o & 2147483644;
    qa = 0L;
    b = w;
    loop L_ud {
      b[0]:int = (qa = b[0]:uint * 10L + qa);
      o = b + 4;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      o = b + 8;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      o = b + 12;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      qa = qa >> 32L;
      b = b + 16;
      i = i + -4;
      if (i) continue L_ud;
    }
    label B_td:
    if (eqz(g)) goto B_vd;
    loop L_wd {
      b[0]:int = (qa = b[0]:uint * 10L + qa);
      b = b + 4;
      qa = qa >> 32L;
      g = g + -1;
      if (g) continue L_wd;
    }
    label B_vd:
    b = i32_wrap_i64(qa);
    if (eqz(b)) goto B_rd;
    if (q > 39) goto B_gb;
    (e + (q << 2) + 4)[0]:int = b;
    q = q + 1;
    label B_rd:
    e[0]:int = q;
    if (n >= 41) goto B_fb;
    if (n) goto B_yd;
    n = 0;
    goto B_xd;
    label B_yd:
    i = n + -1 & 1073741823;
    o = i + 1;
    g = o & 3;
    qa = 0L;
    b = v;
    if (i < 3) goto B_zd;
    i = o & 2147483644;
    qa = 0L;
    b = v;
    loop L_ae {
      b[0]:int = (qa = b[0]:uint * 10L + qa);
      o = b + 4;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      o = b + 8;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      o = b + 12;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      qa = qa >> 32L;
      b = b + 16;
      i = i + -4;
      if (i) continue L_ae;
    }
    label B_zd:
    if (eqz(g)) goto B_be;
    loop L_ce {
      b[0]:int = (qa = b[0]:uint * 10L + qa);
      b = b + 4;
      qa = qa >> 32L;
      g = g + -1;
      if (g) continue L_ce;
    }
    label B_be:
    b = i32_wrap_i64(qa);
    if (eqz(b)) goto B_xd;
    if (n > 39) goto B_eb;
    (e + 168 + (n << 2) + 4)[0]:int = b;
    n = n + 1;
    label B_xd:
    e[42]:int = n;
    if (oa >= 41) goto B_db;
    if (oa) goto B_ee;
    oa = 0;
    goto B_de;
    label B_ee:
    i = oa + -1 & 1073741823;
    o = i + 1;
    g = o & 3;
    qa = 0L;
    b = u;
    if (i < 3) goto B_fe;
    i = o & 2147483644;
    qa = 0L;
    b = u;
    loop L_ge {
      b[0]:int = (qa = b[0]:uint * 10L + qa);
      o = b + 4;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      o = b + 8;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      o = b + 12;
      o[0]:int = (qa = o[0]:uint * 10L + (qa >> 32L));
      qa = qa >> 32L;
      b = b + 16;
      i = i + -4;
      if (i) continue L_ge;
    }
    label B_fe:
    if (eqz(g)) goto B_he;
    loop L_ie {
      b[0]:int = (qa = b[0]:uint * 10L + qa);
      b = b + 4;
      qa = qa >> 32L;
      g = g + -1;
      if (g) continue L_ie;
    }
    label B_he:
    b = i32_wrap_i64(qa);
    if (eqz(b)) goto B_de;
    if (oa > 39) goto B_cb;
    (e + 336 + (oa << 2) + 4)[0]:int = b;
    oa = oa + 1;
    label B_de:
    e[84]:int = oa;
    m = select_if(q, aa, q > aa);
    if (m <= 40) continue L_rb;
  }
  label B_qb:
  f_sj(m, 40, e);
  unreachable;
  label B_pb:
  if (g >= f) goto B_a;
  if (m >= f) goto B_je;
  f_q(e, 1);
  b = e[0]:int;
  b = select_if(b, g = e[126]:int, b > g);
  if (b >= 41) goto B_bb;
  b = b << 2;
  loop L_le {
    if (b) goto B_me;
    g = select_if(-1, 0, b);
    goto B_ke;
    label B_me:
    g = e + 504 + b;
    i = e + b;
    b = b + -4;
    g = select_if(-1, (i = i[0]:int) != (g = g[0]), i < g);
    if (eqz(g)) continue L_le;
  }
  label B_ke:
  if (g >= 2) goto B_a;
  label B_je:
  if (na >= d) goto B_ab;
  p = c + ma;
  g = -1;
  b = na;
  loop L_oe {
    if (b == -1) goto B_ne;
    g = g + 1;
    i = c + b;
    o = b + -1;
    b = o;
    if (i[0]:ubyte == 57) continue L_oe;
  }
  i = c + o;
  b = i + 1;
  b[0]:byte = b[0]:ubyte + 1;
  if (na < o + 2) goto B_a;
  f_bk(i + 2, 48, g);
  goto B_a;
  label B_ne:
  c[0] = 49;
  if (eqz(na)) goto B_pe;
  f_bk(c + 1, 48, na);
  label B_pe:
  if (ma >= d) goto B_qe;
  p[0] = 48;
  h = h + 1;
  ma = na + 2;
  goto B_a;
  label B_qe:
  f_ne(ma, d, 1070100);
  unreachable;
  label B_ob:
  f_sj(q, 40, e);
  unreachable;
  label B_nb:
  f_rf(1075512, 26, 1075496);
  unreachable;
  label B_mb:
  f_sj(b, 40, e);
  unreachable;
  label B_lb:
  f_sj(l, 40, e);
  unreachable;
  label B_kb:
  f_ne(l, 40, 1075496);
  unreachable;
  label B_jb:
  f_sj(b, 40, e);
  unreachable;
  label B_ib:
  f_ne(d, d, 1070084);
  unreachable;
  label B_hb:
  f_sj(q, 40, e);
  unreachable;
  label B_gb:
  f_ne(q, 40, 1075496);
  unreachable;
  label B_fb:
  f_sj(n, 40, e);
  unreachable;
  label B_eb:
  f_ne(n, 40, 1075496);
  unreachable;
  label B_db:
  f_sj(oa, 40, e);
  unreachable;
  label B_cb:
  f_ne(oa, 40, 1075496);
  unreachable;
  label B_bb:
  f_sj(b, 40, e);
  unreachable;
  label B_ab:
  f_sj(ma, d, e);
  unreachable;
  label B_b:
  f_ne(l, 40, 1075496);
  unreachable;
  label B_a:
  if (ma > d) goto B_re;
  a.c = h;
  a.b = ma;
  a.a = c;
  g_a = e + 1344;
  return ;
  label B_re:
  f_sj(ma, d, e);
  unreachable;
}

function f_b(a:{ a:int, b:int, c:short }, b:int, c:byte_ptr, d:int, e:int) {
  var g:int;
  var i:int;
  var k:int;
  var n:int;
  var o:int;
  var p:int;
  var s:byte_ptr;
  var t:int;
  var m:int;
  var q:int;
  var ja:long;
  var h:int;
  var j:int;
  var l:int;
  var r:int;
  var ga:int;
  var ha:int;
  var ia:int;
  var f:int = g_a - 848;
  g_a = f;
  ja = b[0]:long;
  if (eqz(ja)) goto B_n;
  var ka:long = b[1]:long;
  if (eqz(ka)) goto B_m;
  var la:long = b[2]:long;
  if (eqz(la)) goto B_l;
  if (ja + la < ja) goto B_k;
  if (ja - ka > ja) goto B_j;
  b = b[12]:ushort;
  f[3]:int = ja;
  (f + 8 + 8)[0]:int =
    select_if(0, i32_wrap_i64(ja >> 32L), g = ja < 4294967296L);
  f[2]:int = select_if(1, 2, g);
  f_bk(f + 20, 0, 152);
  f_bk(f + 176 + 8, 0, 156);
  f[22]:long = 4294967297L;
  g = 
    i32_wrap_i64(
      (((i64_extend_i32_u(b) << 48L) >> 48L) - clz(ja + -1L)) * 1292913986L +
      82746495104L >>
      32L);
  h = (g << 16) >> 16;
  i = (b << 16) >> 16;
  if (i < 0) goto B_p;
  f_q(f + 8, b);
  goto B_o;
  label B_p:
  f_q(f + 176, (0 - i << 16) >> 16);
  label B_o:
  if (h > -1) goto B_r;
  f_d(f + 8, (0 - h << 16) >> 16);
  goto B_q;
  label B_r:
  f_d(f + 176, g & 65535);
  label B_q:
  j = f[44]:int;
  f_dk(f + 680 | 4, k = f + 176 | 4, 160);
  f[170]:int = j;
  l = d;
  if (d < 10) goto B_s;
  if (j <= 40) goto B_t;
  f_sj(j, 40, b);
  unreachable;
  label B_t:
  m = f + 680 + -4;
  l = d;
  b = j;
  loop L_u {
    if (eqz(b)) goto B_v;
    i = b << 2;
    b = b + -1 & 1073741823;
    g = b + 1;
    n = g & 1;
    if (b) goto B_x;
    b = f + 680 + i + 4;
    ja = 0L;
    goto B_w;
    label B_x:
    g = g & 2147483646;
    b = m + i;
    ja = 0L;
    loop L_y {
      i = b + 4;
      i[0]:int = (la = (ja = ja << 32L | i[0]:uint) / 1000000000L);
      b[0]:int =
        (la = (ja = ja - la * 1000000000L << 32L | b[0]:uint) / 1000000000L);
      ja = ja - la * 1000000000L;
      b = b + -8;
      g = g + -2;
      if (g) continue L_y;
    }
    b = b + 8;
    label B_w:
    if (eqz(n)) goto B_v;
    b = b + -4;
    b[0]:int = (ja << 32L | b[0]:uint) / 1000000000L;
    label B_v:
    l = l + -9;
    if (l <= 9) goto B_s;
    b = f[170]:int;
    if (b < 41) continue L_u;
  }
  f_sj(b, 40, b);
  unreachable;
  label B_s:
  g = ((l << 2) + 1069436)[0]:int;
  if (eqz(g)) goto B_ca;
  b = f[170]:int;
  if (b >= 41) goto B_i;
  if (b) goto B_da;
  b = 0;
  goto B_z;
  label B_da:
  i = b << 2;
  b = b + -1 & 1073741823;
  n = b + 1;
  l = n & 1;
  ja = i64_extend_i32_u(g);
  if (b) goto B_ba;
  b = f + 680 + i + 4;
  la = 0L;
  goto B_aa;
  label B_ca:
  f_rf(1075567, 27, 1075496);
  unreachable;
  label B_ba:
  g = n & 2147483646;
  b = i + f + 680 + -4;
  la = 0L;
  loop L_ea {
    i = b + 4;
    i[0]:int = (ka = (la = la << 32L | i[0]:uint) / ja);
    b[0]:int = (ka = (la = la - ka * ja << 32L | b[0]:uint) / ja);
    la = la - ka * ja;
    b = b + -8;
    g = g + -2;
    if (g) continue L_ea;
  }
  b = b + 8;
  label B_aa:
  if (eqz(l)) goto B_fa;
  b = b + -4;
  b[0]:int = (la << 32L | b[0]:uint) / ja;
  label B_fa:
  b = f[170]:int;
  label B_z:
  p = select_if(b, o = f[2]:int, b > o);
  if (p > 40) goto B_ja;
  if (p) goto B_ka;
  p = 0;
  goto B_ga;
  label B_ka:
  q = p & 1;
  if (p != 1) goto B_ia;
  l = 0;
  n = 0;
  goto B_ha;
  label B_ja:
  f_sj(p, 40, b);
  unreachable;
  label B_ia:
  r = p & -2;
  g = f + 8 + 8;
  b = f + 680 + 8;
  l = 0;
  n = 0;
  loop L_la {
    i = b + -4;
    i[0]:int = (s = (i = (m = i[0]:int) + (g + -4)[0]:int) + (l & 1));
    b[0]:int = (i = (l = (t = b[0]:int) + g[0]:int) + (i < m | s < i));
    l = l < t | i < l;
    g = g + 8;
    b = b + 8;
    if (r != (n = n + 2)) continue L_la;
  }
  label B_ha:
  if (eqz(q)) goto B_ma;
  g = f + 680 + (b = n << 2) + 4;
  g[0]:int = (i = (b = (g = g[0]:int) + (f + 8 + b + 4)[0]:int) + l);
  l = b < g | i < b;
  label B_ma:
  if (eqz(l & 1)) goto B_ga;
  if (p > 39) goto B_h;
  (f + 680 + (p << 2) + 4)[0]:int = 1;
  p = p + 1;
  label B_ga:
  f[170]:int = p;
  g = select_if(p, j, p > j);
  if (g >= 41) goto B_g;
  b = f + 176 | 4;
  q = f + 8 | 4;
  g = g << 2;
  loop L_oa {
    if (g) goto B_pa;
    i = select_if(-1, 0, g);
    goto B_na;
    label B_pa:
    i = f + 176 + g;
    l = f + 680 + g;
    g = g + -4;
    i = select_if(-1, (l = l[0]:int) != (i = i[0]:int), l < i);
    if (eqz(i)) continue L_oa;
  }
  label B_na:
  if (i < 2) goto B_qa;
  if (o >= 41) goto B_f;
  if (o) goto B_ra;
  f[2]:int = 0;
  goto B_d;
  label B_ra:
  g = o + -1 & 1073741823;
  l = g + 1;
  i = l & 3;
  if (g >= 3) goto B_sa;
  ja = 0L;
  g = q;
  goto B_e;
  label B_sa:
  l = l & 2147483644;
  ja = 0L;
  g = q;
  loop L_ta {
    g[0]:int = (ja = g[0]:uint * 10L + ja);
    n = g + 4;
    n[0]:int = (ja = n[0]:uint * 10L + (ja >> 32L));
    n = g + 8;
    n[0]:int = (ja = n[0]:uint * 10L + (ja >> 32L));
    n = g + 12;
    n[0]:int = (ja = n[0]:uint * 10L + (ja >> 32L));
    ja = ja >> 32L;
    g = g + 16;
    l = l + -4;
    if (l) continue L_ta;
    goto B_e;
  }
  label B_qa:
  h = h + 1;
  goto B_d;
  label B_n:
  f_rf(1069739, 28, 1070116);
  unreachable;
  label B_m:
  f_rf(1069784, 29, 1070132);
  unreachable;
  label B_l:
  f_rf(1069832, 28, 1070148);
  unreachable;
  label B_k:
  f_rf(1069876, 54, 1070164);
  unreachable;
  label B_j:
  f_rf(1069948, 55, 1070180);
  unreachable;
  label B_i:
  f_sj(b, 40, b);
  unreachable;
  label B_h:
  f_ne(p, 40, 1075496);
  unreachable;
  label B_g:
  f_sj(g, 40, b);
  unreachable;
  label B_f:
  f_sj(o, 40, b);
  unreachable;
  label B_e:
  if (eqz(i)) goto B_ua;
  loop L_va {
    g[0]:int = (ja = g[0]:uint * 10L + ja);
    g = g + 4;
    ja = ja >> 32L;
    i = i + -1;
    if (i) continue L_va;
  }
  label B_ua:
  g = i32_wrap_i64(ja);
  if (eqz(g)) goto B_wa;
  if (o > 39) goto B_c;
  (f + 8 + (o << 2) + 4)[0]:int = g;
  o = o + 1;
  label B_wa:
  f[2]:int = o;
  label B_d:
  m = 1;
  g = (h << 16) >> 16;
  if (g < (i = (e << 16) >> 16)) goto B_ya;
  n = select_if((h - e << 16) >> 16, d, g - i < d);
  if (n) goto B_xa;
  label B_ya:
  n = 0;
  goto B_b;
  label B_xa:
  f_dk(f + 344 | 4, k, 160);
  f[86]:int = j;
  var u:int = f_q(f + 344, 1);
  g = f[44]:int;
  f_dk(f + 512 | 4, k, 160);
  f[128]:int = g;
  var v:int = f_q(f + 512, 2);
  g = f[44]:int;
  f_dk(f + 680 | 4, k, 160);
  f[170]:int = g;
  var w:int = f + 176 + 8;
  var x:int = f + 344 + 8;
  var y:int = f + 512 + 8;
  var z:int = f + 680 + 8;
  var aa:int = f + 8 + 8;
  var ba:int = f_q(f + 680, 3);
  s = f[2]:int;
  j = f[44]:int;
  var ca:int = f[86]:int;
  var da:int = f[128]:int;
  var ea:int = f[170]:int;
  var fa:int = 0;
  loop L_eb {
    k = fa;
    if (s >= 41) goto B_lb;
    fa = k + 1;
    g = s << 2;
    i = q;
    loop L_pb {
      if (eqz(g)) goto B_ob;
      g = g + -4;
      l = i[0]:int;
      i = i + 4;
      if (eqz(l)) continue L_pb;
    }
    ga = select_if(s, ea, s > ea);
    if (ga >= 41) goto B_kb;
    g = ga << 2;
    loop L_rb {
      if (g) goto B_sb;
      i = select_if(-1, 0, g);
      goto B_qb;
      label B_sb:
      i = f + 680 + g;
      l = f + 8 + g;
      g = g + -4;
      i = select_if(-1, (l = l[0]:int) != (i = i[0]:int), l < i);
      if (eqz(i)) continue L_rb;
    }
    label B_qb:
    ha = 0;
    if (i >= 2) goto B_mb;
    if (eqz(ga)) goto B_nb;
    m = 1;
    ha = ga & 1;
    s = 0;
    if (ga == 1) goto B_tb;
    o = ga & -2;
    s = 0;
    m = 1;
    i = z;
    g = aa;
    loop L_ub {
      l = g + -4;
      l[0]:int =
        (r = (l = (t = l[0]:int) + ((i + -4)[0]:int ^ -1)) + (m & 1));
      g[0]:int =
        (l = (m = (p = g[0]:int) + (i[0]:int ^ -1)) + (l < t | r < l));
      m = m < p | l < m;
      i = i + 8;
      g = g + 8;
      if (o != (s = s + 2)) continue L_ub;
    }
    label B_tb:
    if (eqz(ha)) goto B_vb;
    i = f + 8 + (g = s << 2) + 4;
    i[0]:int =
      (l = (g = (i = i[0]:int) + ((ba + g + 4)[0]:int ^ -1)) + m);
    m = g < i | l < g;
    label B_vb:
    if (m & 1) goto B_nb;
    f_rf(1075512, 26, 1075496);
    unreachable;
    label B_ob:
    if (n < k) goto B_jb;
    if (n > d) goto B_ib;
    if (n == k) goto B_a;
    f_bk(c + k, 48, n - k);
    goto B_a;
    label B_nb:
    f[2]:int = ga;
    ha = 8;
    s = ga;
    label B_mb:
    o = select_if(s, da, s > da);
    if (o >= 41) goto B_hb;
    g = o << 2;
    loop L_xb {
      if (g) goto B_yb;
      i = select_if(-1, 0, g);
      goto B_wb;
      label B_yb:
      i = f + 512 + g;
      l = f + 8 + g;
      g = g + -4;
      i = select_if(-1, (l = l[0]:int) != (i = i[0]:int), l < i);
      if (eqz(i)) continue L_xb;
    }
    label B_wb:
    if (i < 2) goto B_zb;
    o = s;
    goto B_fb;
    label B_zb:
    if (eqz(o)) goto B_ac;
    m = 1;
    ia = o & 1;
    s = 0;
    if (o == 1) goto B_bc;
    ga = o & -2;
    s = 0;
    m = 1;
    i = y;
    g = aa;
    loop L_cc {
      l = g + -4;
      l[0]:int =
        (r = (l = (t = l[0]:int) + ((i + -4)[0]:int ^ -1)) + (m & 1));
      g[0]:int =
        (l = (m = (p = g[0]:int) + (i[0]:int ^ -1)) + (l < t | r < l));
      m = m < p | l < m;
      i = i + 8;
      g = g + 8;
      if (ga != (s = s + 2)) continue L_cc;
    }
    label B_bc:
    if (eqz(ia)) goto B_dc;
    i = f + 8 + (g = s << 2) + 4;
    i[0]:int = (l = (g = (i = i[0]:int) + ((v + g + 4)[0]:int ^ -1)) + m);
    m = g < i | l < g;
    label B_dc:
    if (eqz(m & 1)) goto B_gb;
    label B_ac:
    f[2]:int = o;
    ha = ha | 4;
    goto B_fb;
    label B_lb:
    f_sj(s, 40, b);
    unreachable;
    label B_kb:
    f_sj(ga, 40, b);
    unreachable;
    label B_jb:
    f_tj(k, n, b);
    unreachable;
    label B_ib:
    f_sj(n, d, b);
    unreachable;
    label B_hb:
    f_sj(o, 40, b);
    unreachable;
    label B_gb:
    f_rf(1075512, 26, 1075496);
    unreachable;
    label B_fb:
    ga = select_if(o, ca, o > ca);
    if (ga >= 41) goto B_gc;
    g = ga << 2;
    loop L_ic {
      if (g) goto B_jc;
      i = select_if(-1, 0, g);
      goto B_hc;
      label B_jc:
      i = f + 344 + g;
      l = f + 8 + g;
      g = g + -4;
      i = select_if(-1, (l = l[0]:int) != (i = i[0]:int), l < i);
      if (eqz(i)) continue L_ic;
    }
    label B_hc:
    if (i < 2) goto B_kc;
    ga = o;
    goto B_ec;
    label B_kc:
    if (eqz(ga)) goto B_lc;
    m = 1;
    ia = ga & 1;
    s = 0;
    if (ga == 1) goto B_mc;
    o = ga & -2;
    s = 0;
    m = 1;
    i = x;
    g = aa;
    loop L_nc {
      l = g + -4;
      l[0]:int =
        (r = (l = (t = l[0]:int) + ((i + -4)[0]:int ^ -1)) + (m & 1));
      g[0]:int =
        (l = (m = (p = g[0]:int) + (i[0]:int ^ -1)) + (l < t | r < l));
      m = m < p | l < m;
      i = i + 8;
      g = g + 8;
      if (o != (s = s + 2)) continue L_nc;
    }
    label B_mc:
    if (eqz(ia)) goto B_oc;
    i = f + 8 + (g = s << 2) + 4;
    i[0]:int = (l = (g = (i = i[0]:int) + ((u + g + 4)[0]:int ^ -1)) + m);
    m = g < i | l < g;
    label B_oc:
    if (eqz(m & 1)) goto B_fc;
    label B_lc:
    f[2]:int = ga;
    ha = ha + 2;
    goto B_ec;
    label B_gc:
    f_sj(ga, 40, b);
    unreachable;
    label B_fc:
    f_rf(1075512, 26, 1075496);
    unreachable;
    label B_ec:
    s = select_if(ga, j, ga > j);
    if (s >= 41) goto B_cb;
    g = s << 2;
    loop L_qc {
      if (g) goto B_rc;
      i = select_if(-1, 0, g);
      goto B_pc;
      label B_rc:
      i = f + 176 + g;
      l = f + 8 + g;
      g = g + -4;
      i = select_if(-1, (l = l[0]:int) != (i = i[0]:int), l < i);
      if (eqz(i)) continue L_qc;
    }
    label B_pc:
    if (i < 2) goto B_tc;
    s = ga;
    goto B_sc;
    label B_tc:
    if (eqz(s)) goto B_uc;
    m = 1;
    ia = s & 1;
    t = 0;
    if (s == 1) goto B_vc;
    ga = s & -2;
    t = 0;
    m = 1;
    i = w;
    g = aa;
    loop L_wc {
      l = g + -4;
      l[0]:int =
        (p = (l = (r = l[0]:int) + ((i + -4)[0]:int ^ -1)) + (m & 1));
      g[0]:int =
        (l = (m = (o = g[0]:int) + (i[0]:int ^ -1)) + (l < r | p < l));
      m = m < o | l < m;
      i = i + 8;
      g = g + 8;
      if (ga != (t = t + 2)) continue L_wc;
    }
    label B_vc:
    if (eqz(ia)) goto B_xc;
    i = f + 8 + (g = t << 2) + 4;
    i[0]:int =
      (l = (g = (i = i[0]:int) + ((f + 176 + g + 4)[0]:int ^ -1)) + m);
    m = g < i | l < g;
    label B_xc:
    if (eqz(m & 1)) goto B_bb;
    label B_uc:
    f[2]:int = s;
    ha = ha + 1;
    label B_sc:
    if (k == d) goto B_db;
    (c + k)[0]:byte = ha + 48;
    if (s >= 41) goto B_ab;
    if (s) goto B_zc;
    s = 0;
    goto B_yc;
    label B_zc:
    l = s + -1 & 1073741823;
    m = l + 1;
    i = m & 3;
    ja = 0L;
    g = q;
    if (l < 3) goto B_ad;
    l = m & 2147483644;
    ja = 0L;
    g = q;
    loop L_bd {
      g[0]:int = (ja = g[0]:uint * 10L + ja);
      m = g + 4;
      m[0]:int = (ja = m[0]:uint * 10L + (ja >> 32L));
      m = g + 8;
      m[0]:int = (ja = m[0]:uint * 10L + (ja >> 32L));
      m = g + 12;
      m[0]:int = (ja = m[0]:uint * 10L + (ja >> 32L));
      ja = ja >> 32L;
      g = g + 16;
      l = l + -4;
      if (l) continue L_bd;
    }
    label B_ad:
    if (eqz(i)) goto B_cd;
    loop L_dd {
      g[0]:int = (ja = g[0]:uint * 10L + ja);
      g = g + 4;
      ja = ja >> 32L;
      i = i + -1;
      if (i) continue L_dd;
    }
    label B_cd:
    g = i32_wrap_i64(ja);
    if (eqz(g)) goto B_yc;
    if (s > 39) goto B_za;
    (f + 8 + (s << 2) + 4)[0]:int = g;
    s = s + 1;
    label B_yc:
    f[2]:int = s;
    if (fa != n) continue L_eb;
  }
  m = 0;
  goto B_b;
  label B_db:
  f_ne(d, d, 1070196);
  unreachable;
  label B_cb:
  f_sj(s, 40, b);
  unreachable;
  label B_bb:
  f_rf(1075512, 26, 1075496);
  unreachable;
  label B_ab:
  f_sj(s, 40, b);
  unreachable;
  label B_za:
  f_ne(s, 40, 1075496);
  unreachable;
  label B_c:
  f_ne(o, 40, 1075496);
  unreachable;
  label B_b:
  if (j >= 41) goto B_ld;
  if (j) goto B_md;
  j = 0;
  goto B_jd;
  label B_md:
  i = j + -1 & 1073741823;
  l = i + 1;
  g = l & 3;
  if (i >= 3) goto B_nd;
  ja = 0L;
  goto B_kd;
  label B_nd:
  i = l & 2147483644;
  ja = 0L;
  loop L_od {
    b[0]:int = (ja = b[0]:uint * 5L + ja);
    l = b + 4;
    l[0]:int = (ja = l[0]:uint * 5L + (ja >> 32L));
    l = b + 8;
    l[0]:int = (ja = l[0]:uint * 5L + (ja >> 32L));
    l = b + 12;
    l[0]:int = (ja = l[0]:uint * 5L + (ja >> 32L));
    ja = ja >> 32L;
    b = b + 16;
    i = i + -4;
    if (i) continue L_od;
    goto B_kd;
  }
  label B_ld:
  f_sj(j, 40, b);
  unreachable;
  label B_kd:
  if (eqz(g)) goto B_pd;
  loop L_qd {
    b[0]:int = (ja = b[0]:uint * 5L + ja);
    b = b + 4;
    ja = ja >> 32L;
    g = g + -1;
    if (g) continue L_qd;
  }
  label B_pd:
  b = i32_wrap_i64(ja);
  if (eqz(b)) goto B_jd;
  if (j > 39) goto B_id;
  (f + 176 + (j << 2) + 4)[0]:int = b;
  j = j + 1;
  label B_jd:
  f[44]:int = j;
  b = f[2]:int;
  b = select_if(b, j, b > j);
  if (b >= 41) goto B_hd;
  b = b << 2;
  loop L_sd {
    if (eqz(b)) goto B_rd;
    g = f + 176 + b;
    i = f + 8 + b;
    b = b + -4;
    g = select_if(-1, (i = i[0]:int) != (g = g[0]:int), i < g);
    if (eqz(g)) continue L_sd;
  }
  if ((g & 255) != 1) goto B_ed;
  goto B_fd;
  label B_rd:
  if (b) goto B_ed;
  if (m) goto B_fd;
  b = n + -1;
  if (b >= d) goto B_gd;
  if ((c + b)[0]:ubyte & 1) goto B_fd;
  goto B_ed;
  label B_id:
  f_ne(j, 40, 1075496);
  unreachable;
  label B_hd:
  f_sj(b, 40, b);
  unreachable;
  label B_gd:
  f_ne(b, d, 1070212);
  unreachable;
  label B_fd:
  if (n > d) goto B_td;
  s = c + n;
  b = 0;
  g = c;
  loop L_vd {
    if (n == b) goto B_ud;
    b = b + 1;
    i = g + n;
    l = g + -1;
    g = l;
    if ((i + -1)[0]:ubyte == 57) continue L_vd;
  }
  g = l + n;
  g[0]:byte = g[0]:ubyte + 1;
  if (n <= n - b + 1) goto B_ed;
  f_bk(g + 1, 48, b + -1);
  goto B_ed;
  label B_ud:
  b = 49;
  if (m) goto B_wd;
  c[0] = 49;
  b = 48;
  if (n == 1) goto B_wd;
  b = 48;
  f_bk(c + 1, 48, n + -1);
  label B_wd:
  h = (h << 16) + 65536 >> 16;
  if (h <= (e << 16) >> 16) goto B_ed;
  if (n >= d) goto B_ed;
  s[0] = b;
  n = n + 1;
  goto B_ed;
  label B_td:
  f_sj(n, d, b);
  unreachable;
  label B_ed:
  if (n <= d) goto B_a;
  f_sj(n, d, b);
  unreachable;
  label B_a:
  a.c = h;
  a.b = n;
  a.a = c;
  g_a = f + 848;
}

function f_c(a:int, b:int_ptr):int {
  var h:long;
  var e:int;
  var i:long;
  var j:long;
  var k:double;
  var f:int;
  var c:int = g_a - 112;
  g_a = c;
  var d:int = f_ji(b);
  c[5]:int = b;
  if (d) goto B_s;
  c[24]:int = c + 20;
  br_table[B_z, B_y, B_x, B_w, B_u, B_v, ..B_z](a[0]:ubyte)
  label B_z:
  f_jc(c + 24, c + 20, 1054200, 4);
  a = 0;
  if (c[24]:ubyte == 4) goto B_e;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_e;
  label B_y:
  if (a[1]:ubyte) goto B_ba;
  f_jc(c + 24, c + 20, 1054208, 5);
  goto B_aa;
  label B_ba:
  f_jc(c + 24, c + 20, 1054204, 4);
  label B_aa:
  a = 0;
  if (c[24]:ubyte == 4) goto B_e;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_e;
  label B_x:
  br_table[B_ea, B_da, B_ca, ..B_ea]((a + 8)[0]:int)
  label B_ea:
  b = 20;
  h = (a + 16)[0]:long;
  if (h >= 10000L) goto B_fa;
  i = h;
  goto B_f;
  label B_fa:
  b = 20;
  loop L_ga {
    a = c + 56 + b;
    (a + -4)[0]:short@1 =
      (((e = ((d = i32_wrap_i64(h - (i = h / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((d - e * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    b = b + -4;
    a = h > 99999999L;
    h = i;
    if (a) continue L_ga;
    goto B_f;
  }
  label B_da:
  b = 20;
  j = (a + 16)[0]:long;
  h = j + (h = j >> 63L) ^ h;
  if (h >= 10000L) goto B_ha;
  i = h;
  goto B_g;
  label B_ha:
  b = 20;
  loop L_ia {
    a = c + 56 + b;
    (a + -4)[0]:short@1 =
      (((e = ((d = i32_wrap_i64(h - (i = h / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((d - e * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    b = b + -4;
    a = h > 99999999L;
    h = i;
    if (a) continue L_ia;
    goto B_g;
  }
  label B_ca:
  k = (a + 16)[0]:double;
  if ((f_qf(k) & 255) < 2) goto B_ja;
  f_jc(c + 24, c + 20, c + 56, f_w(k, c + 56));
  if (c[24]:ubyte == 4) goto B_d;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_c;
  label B_ja:
  f_jc(c + 24, c + 20, 1054200, 4);
  if (c[24]:ubyte == 4) goto B_d;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_c;
  label B_w:
  f_sa(c + 24, c + 96, c + 96, (a + 4)[0]:int, (a + 12)[0]:int);
  a = 0;
  if (c[24]:ubyte == 4) goto B_e;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_e;
  label B_v:
  d = (a + 12)[0]:int;
  b = 1;
  f_jc(c + 24, c + 20, 1054213, 1);
  if (c[24]:ubyte != 4) goto B_t;
  if (d) goto B_ka;
  f_jc(c + 24, c + 20, 1054214, 1);
  if (c[24]:ubyte != 4) goto B_r;
  b = 0;
  label B_ka:
  c[108]:byte = b;
  d = (a + 8)[0]:int;
  c[26]:int = c + 96;
  b = a[3]:int;
  a = a[1]:int;
  (c + 80)[0]:int = d;
  (c + 76)[0]:int = a;
  (c + 56 + 8)[0]:int = d;
  c[15]:int = a;
  c[18]:int = (a = eqz(d) << 1);
  c[14]:int = a;
  if (eqz(d)) goto B_h;
  if (eqz(b)) goto B_h;
  e = c + 56 | 4;
  loop L_la {
    c[22]:int = b + -1;
    br_table[B_na, B_ma, B_q, ..B_ma](c[14]:int)
    label B_na:
    b = c[16]:int;
    a = c[15]:int;
    if (eqz(a)) goto B_oa;
    f = a + -1;
    d = a & 7;
    if (eqz(d)) goto B_pa;
    loop L_qa {
      a = a + -1;
      b = b[102];
      d = d + -1;
      if (d) continue L_qa;
    }
    label B_pa:
    if (f < 7) goto B_oa;
    loop L_ra {
      b = 
        (((((((b[102])[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
      a = a + -8;
      if (a) continue L_ra;
    }
    label B_oa:
    c[17]:int = 0;
    c[16]:int = b;
    c[7]:long = 1L;
    label B_ma:
    f_sc(c + 8, e);
    a = c[2]:int;
    if (eqz(a)) goto B_h;
    a = f_md(c + 104, a, c[3]:int);
    if (a) goto B_c;
    b = c[22]:int;
    if (b) continue L_la;
    goto B_h;
  }
  label B_u:
  a = f_kc(c + 96, a + 4);
  goto B_e;
  label B_t:
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_c;
  label B_s:
  b = 0;
  (c + 40)[0]:byte = 0;
  (c + 24 + 12)[0]:int = 2;
  (c + 24 + 8)[0]:int = 1054422;
  c[7]:int = 0;
  c[6]:int = c + 20;
  br_table[B_ya, B_xa, B_wa, B_va, B_ta, B_ua, ..B_ya](a[0]:ubyte)
  label B_ya:
  f_jc(c + 104, c + 20, 1054200, 4);
  b = 0;
  if (c[104]:ubyte == 4) goto B_k;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_k;
  label B_xa:
  if (a[1]:ubyte) goto B_ab;
  f_jc(c + 104, c + 20, 1054208, 5);
  goto B_za;
  label B_ab:
  f_jc(c + 104, c + 20, 1054204, 4);
  label B_za:
  b = 0;
  if (c[104]:ubyte == 4) goto B_k;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_k;
  label B_wa:
  br_table[B_db, B_cb, B_bb, ..B_db]((a + 8)[0]:int)
  label B_db:
  b = 20;
  h = (a + 16)[0]:long;
  if (h >= 10000L) goto B_eb;
  i = h;
  goto B_l;
  label B_eb:
  b = 20;
  loop L_fb {
    a = c + 56 + b;
    (a + -4)[0]:short@1 =
      (((e = ((d = i32_wrap_i64(h - (i = h / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((d - e * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    b = b + -4;
    a = h > 99999999L;
    h = i;
    if (a) continue L_fb;
    goto B_l;
  }
  label B_cb:
  b = 20;
  j = (a + 16)[0]:long;
  h = j + (h = j >> 63L) ^ h;
  if (h >= 10000L) goto B_gb;
  i = h;
  goto B_m;
  label B_gb:
  b = 20;
  loop L_hb {
    a = c + 56 + b;
    (a + -4)[0]:short@1 =
      (((e = ((d = i32_wrap_i64(h - (i = h / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((d - e * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    b = b + -4;
    a = h > 99999999L;
    h = i;
    if (a) continue L_hb;
    goto B_m;
  }
  label B_bb:
  k = (a + 16)[0]:double;
  if ((f_qf(k) & 255) < 2) goto B_ib;
  f_jc(c + 104, c + 20, c + 56, f_w(k, c + 56));
  if (c[104]:ubyte == 4) goto B_j;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_i;
  label B_ib:
  f_jc(c + 104, c + 20, 1054200, 4);
  if (c[104]:ubyte == 4) goto B_j;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_i;
  label B_va:
  f_ta(c + 104, c + 24, c + 24 | 4, (a + 4)[0]:int, (a + 12)[0]:int);
  if (c[104]:ubyte == 4) goto B_k;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_k;
  label B_ua:
  c[40]:byte = 0;
  f = 1;
  c[7]:int = 1;
  b = (a + 12)[0]:int;
  f_jc(c + 104, c + 20, 1054213, 1);
  if (c[104]:ubyte != 4) goto B_sa;
  if (b) goto B_jb;
  f = 0;
  c[7]:int = 0;
  f_jc(c + 104, c + 20, 1054214, 1);
  if (c[104]:ubyte == 4) goto B_jb;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_i;
  label B_jb:
  c[52]:byte = f;
  d = (a + 8)[0]:int;
  c[12]:int = c + 24;
  b = a[3]:int;
  a = a[1]:int;
  (c + 80)[0]:int = d;
  (c + 76)[0]:int = a;
  (c + 56 + 8)[0]:int = d;
  c[15]:int = a;
  c[18]:int = (a = eqz(d) << 1);
  c[14]:int = a;
  e = c + 24;
  if (eqz(d)) goto B_n;
  if (eqz(b)) goto B_n;
  f = c + 56 | 4;
  e = c + 24;
  loop L_kb {
    c[22]:int = b + -1;
    br_table[B_mb, B_lb, B_p, ..B_lb](a)
    label B_mb:
    b = c[16]:int;
    a = c[15]:int;
    if (eqz(a)) goto B_nb;
    var g:int = a + -1;
    d = a & 7;
    if (eqz(d)) goto B_ob;
    loop L_pb {
      a = a + -1;
      b = b[102];
      d = d + -1;
      if (d) continue L_pb;
    }
    label B_ob:
    if (g < 7) goto B_nb;
    loop L_qb {
      b = 
        (((((((b[102])[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
      a = a + -8;
      if (a) continue L_qb;
    }
    label B_nb:
    c[17]:int = 0;
    c[16]:int = b;
    c[7]:long = 1L;
    label B_lb:
    f_sc(c, f);
    a = c[0]:int;
    if (eqz(a)) goto B_o;
    d = c[1]:int;
    b = f_vc(c + 48, a);
    if (b) goto B_i;
    f_jc(c + 96, (e = c[12]:int)[0]:int, 1054427, 2);
    if (c[96]:ubyte == 4) goto B_rb;
    c[13]:long = c[12]:long;
    b = f_cg(c + 104);
    goto B_i;
    label B_rb:
    b = f_i(d, e);
    if (b) goto B_i;
    e[16]:byte = 1;
    b = c[22]:int;
    if (eqz(b)) goto B_o;
    a = c[14]:int;
    continue L_kb;
  }
  label B_ta:
  b = f_qa(c + 24, a + 4);
  goto B_k;
  label B_sa:
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_i;
  label B_r:
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_c;
  label B_q:
  f_rf(1055948, 43, 1056088);
  unreachable;
  label B_p:
  f_rf(1055948, 43, 1056088);
  unreachable;
  label B_o:
  f = c[52]:ubyte;
  label B_n:
  if (f & 255) goto B_sb;
  b = 0;
  goto B_k;
  label B_sb:
  e[1]:int = (a = e[1]:int + -1);
  if (eqz(e[16]:ubyte)) goto B_ub;
  f_jc(c + 56, e[0]:int, 1054215, 1);
  if (c[56]:ubyte != 4) goto B_vb;
  f_af(c + 56, e, a, e[2]:int, e[3]:int);
  if (c[56]:ubyte == 4) goto B_ub;
  c[13]:long = (h = c[7]:long);
  a = i32_wrap_i64(h);
  goto B_tb;
  label B_vb:
  c[13]:long = (h = c[7]:long);
  a = i32_wrap_i64(h);
  goto B_tb;
  label B_ub:
  f_jc(c + 104, e[0]:int, 1054214, 1);
  a = c[104]:ubyte;
  label B_tb:
  b = 0;
  if ((a & 255) == 4) goto B_k;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_k;
  label B_m:
  a = i32_wrap_i64(i);
  if (a > 99) goto B_xb;
  d = a;
  goto B_wb;
  label B_xb:
  (c + 56 + (b = b + -2))[0]:short@1 =
    ((a - (d = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_wb:
  if (d < 10) goto B_zb;
  (c + 56 + (a = b + -2))[0]:short@1 = ((d << 1) + 1054000)[0]:ushort@1;
  goto B_yb;
  label B_zb:
  (c + 56 + (a = b + -1))[0]:byte = d + 48;
  label B_yb:
  if (j > -1L) goto B_ac;
  (c + 56 + (a = a + -1))[0]:byte = 45;
  label B_ac:
  f_jc(c + 104, c + 20, c + 56 + a, 20 - a);
  b = 0;
  if (c[104]:ubyte == 4) goto B_k;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  goto B_k;
  label B_l:
  a = i32_wrap_i64(i);
  if (a > 99) goto B_cc;
  d = a;
  goto B_bc;
  label B_cc:
  (c + 56 + (b = b + -2))[0]:short@1 =
    ((a - (d = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_bc:
  if (d < 10) goto B_ec;
  (c + 56 + (a = b + -2))[0]:short@1 = ((d << 1) + 1054000)[0]:ushort@1;
  goto B_dc;
  label B_ec:
  (c + 56 + (a = b + -1))[0]:byte = d + 48;
  label B_dc:
  f_jc(c + 104, c + 20, c + 56 + a, 20 - a);
  b = 0;
  if (c[104]:ubyte == 4) goto B_k;
  c[7]:long = c[13]:long;
  b = f_cg(c + 56);
  label B_k:
  if (b) goto B_i;
  label B_j:
  a = 0;
  goto B_a;
  label B_i:
  c[14]:int = b;
  f_be(c + 56);
  goto B_b;
  label B_h:
  if (c[108]:ubyte) goto B_fc;
  a = 0;
  goto B_e;
  label B_fc:
  f_jc(c + 24, (c[26]:int)[0]:int, 1054214, 1);
  a = 0;
  if (c[24]:ubyte == 4) goto B_e;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_e;
  label B_g:
  a = i32_wrap_i64(i);
  if (a > 99) goto B_hc;
  d = a;
  goto B_gc;
  label B_hc:
  (c + 56 + (b = b + -2))[0]:short@1 =
    ((a - (d = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_gc:
  if (d < 10) goto B_jc;
  (c + 56 + (a = b + -2))[0]:short@1 = ((d << 1) + 1054000)[0]:ushort@1;
  goto B_ic;
  label B_jc:
  (c + 56 + (a = b + -1))[0]:byte = d + 48;
  label B_ic:
  if (j > -1L) goto B_kc;
  (c + 56 + (a = a + -1))[0]:byte = 45;
  label B_kc:
  f_jc(c + 24, c + 20, c + 56 + a, 20 - a);
  a = 0;
  if (c[24]:ubyte == 4) goto B_e;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  goto B_e;
  label B_f:
  a = i32_wrap_i64(i);
  if (a > 99) goto B_mc;
  d = a;
  goto B_lc;
  label B_mc:
  (c + 56 + (b = b + -2))[0]:short@1 =
    ((a - (d = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_lc:
  if (d < 10) goto B_oc;
  (c + 56 + (a = b + -2))[0]:short@1 = ((d << 1) + 1054000)[0]:ushort@1;
  goto B_nc;
  label B_oc:
  (c + 56 + (a = b + -1))[0]:byte = d + 48;
  label B_nc:
  f_jc(c + 24, c + 20, c + 56 + a, 20 - a);
  a = 0;
  if (c[24]:ubyte == 4) goto B_e;
  c[7]:long = c[3]:long;
  a = f_cg(c + 56);
  label B_e:
  if (a) goto B_c;
  label B_d:
  a = 0;
  goto B_a;
  label B_c:
  c[14]:int = a;
  f_be(c + 56);
  label B_b:
  a = 1;
  label B_a:
  g_a = c + 112;
  return a;
}

function f_d(a:int_ptr, b:int):int {
  var r:long;
  var e:int;
  var h:int_ptr;
  var q:long;
  var f:int;
  var g:int;
  var d:int;
  var j:int;
  var l:int;
  var i:int;
  var k:int;
  var m:int;
  var o:int;
  var c:int = g_a - 160;
  g_a = c;
  d = b & 7;
  if (eqz(d)) goto B_j;
  e = a[0];
  if (e >= 41) goto B_m;
  if (e) goto B_n;
  e = 0;
  goto B_k;
  label B_n:
  q = ((d << 2) + 1069396)[0]:uint;
  d = a + 4;
  f = e + -1 & 1073741823;
  g = f + 1;
  h = g & 3;
  if (f >= 3) goto B_o;
  r = 0L;
  goto B_l;
  label B_o:
  f = g & 2147483644;
  r = 0L;
  loop L_p {
    d[0]:int = (r = d[0]:uint * q + r);
    g = d + 4;
    g[0]:int = (r = g[0]:uint * q + (r >> 32L));
    g = d + 8;
    g[0]:int = (r = g[0]:uint * q + (r >> 32L));
    g = d + 12;
    g[0]:int = (r = g[0]:uint * q + (r >> 32L));
    r = r >> 32L;
    d = d + 16;
    f = f + -4;
    if (f) continue L_p;
    goto B_l;
  }
  label B_m:
  f_sj(e, 40, d);
  unreachable;
  label B_l:
  if (eqz(h)) goto B_q;
  loop L_r {
    d[0]:int = (r = d[0]:uint * q + r);
    d = d + 4;
    r = r >> 32L;
    h = h + -1;
    if (h) continue L_r;
  }
  label B_q:
  d = i32_wrap_i64(r);
  if (eqz(d)) goto B_k;
  if (e > 39) goto B_i;
  (a + (e << 2) + 4)[0]:int = d;
  e = e + 1;
  label B_k:
  a[0] = e;
  label B_j:
  if (eqz(b & 8)) goto B_e;
  e = a[0];
  if (e >= 41) goto B_h;
  if (e) goto B_s;
  e = 0;
  goto B_f;
  label B_s:
  d = a + 4;
  f = e + -1 & 1073741823;
  g = f + 1;
  h = g & 3;
  if (f >= 3) goto B_t;
  q = 0L;
  goto B_g;
  label B_t:
  f = g & 2147483644;
  q = 0L;
  loop L_u {
    d[0]:int = (q = d[0]:uint * 100000000L + q);
    g = d + 4;
    g[0]:int = (q = g[0]:uint * 100000000L + (q >> 32L));
    g = d + 8;
    g[0]:int = (q = g[0]:uint * 100000000L + (q >> 32L));
    g = d + 12;
    g[0]:int = (q = g[0]:uint * 100000000L + (q >> 32L));
    q = q >> 32L;
    d = d + 16;
    f = f + -4;
    if (f) continue L_u;
    goto B_g;
  }
  label B_i:
  f_ne(e, 40, 1075496);
  unreachable;
  label B_h:
  f_sj(e, 40, d);
  unreachable;
  label B_g:
  if (eqz(h)) goto B_v;
  loop L_w {
    d[0]:int = (q = d[0]:uint * 100000000L + q);
    d = d + 4;
    q = q >> 32L;
    h = h + -1;
    if (h) continue L_w;
  }
  label B_v:
  d = i32_wrap_i64(q);
  if (eqz(d)) goto B_f;
  if (e > 39) goto B_d;
  (a + (e << 2) + 4)[0]:int = d;
  e = e + 1;
  label B_f:
  a[0] = e;
  label B_e:
  if (eqz(b & 16)) goto B_a;
  f = 0;
  i = f_bk(c, 0, 160);
  h = a[0];
  if (h < 2) goto B_x;
  if (h >= 41) goto B_c;
  j = f_jb(i, 1069476, 2, a + 4, h);
  goto B_b;
  label B_x:
  d = a + 4;
  e = d + (h << 2);
  k = i + 4;
  j = 0;
  loop L_y {
    h = f + -1;
    f = k + (f << 2);
    loop L_z {
      if (d == e) goto B_b;
      f = f + 4;
      h = h + 1;
      g = d[0]:int;
      l = d + 4;
      d = l;
      if (eqz(g)) continue L_z;
    }
    if (h > 39) goto B_da;
    d = f + -8;
    d[0]:int = (r = (q = i64_extend_i32_u(g)) * 1874919424L + d[0]:uint);
    if (h == 39) goto B_ea;
    d = f + -4;
    d[0]:int = (q = (r >> 32L) + d[0]:uint + q * 2328306L);
    d = i32_wrap_i64(q >> 32L);
    if (d) goto B_ca;
    d = 2;
    goto B_ba;
    label B_ea:
    h = h + 1;
    label B_da:
    f_ne(h, 40, 1075496);
    unreachable;
    label B_ca:
    if (h > 37) goto B_aa;
    f[0]:int = d;
    d = 3;
    label B_ba:
    f = h + 1;
    d = d + h;
    j = select_if(d, j, j < d);
    d = l;
    continue L_y;
    label B_aa:
  }
  f_ne(h + 2, 40, 1075496);
  unreachable;
  label B_d:
  f_ne(e, 40, 1075496);
  unreachable;
  label B_c:
  f_sj(h, 40, d);
  unreachable;
  label B_b:
  f_dk(a + 4, i, 160);
  a[0] = j;
  label B_a:
  if (eqz(b & 32)) goto B_fa;
  j = f_bk(c, 0, 160);
  d = a[0];
  if (d < 4) goto B_ia;
  if (d >= 41) goto B_ha;
  l = f_jb(j, 1069484, 4, a + 4, d);
  goto B_ga;
  label B_ia:
  g = a + 4;
  e = g + (d << 2);
  i = 0;
  l = 0;
  loop L_ja {
    h = i + -1;
    d = 0;
    loop L_ka {
      f = g + d;
      if (f == e) goto B_ga;
      h = h + 1;
      d = d + 4;
      f = f[0]:int;
      if (eqz(f)) continue L_ka;
    }
    if (h > 39) goto B_oa;
    k = select_if(0, k = 40 - h, k > 40);
    if (k == 1) goto B_pa;
    i = j + (i << 2) + d;
    i[0]:int = (r = (q = i64_extend_i32_u(f)) * 2242703233L + i[0]:uint);
    if (k != 2) goto B_qa;
    h = h + 2;
    goto B_oa;
    label B_qa:
    f = i + 4;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 762134875L);
    if (k != 3) goto B_ra;
    h = h + 3;
    goto B_oa;
    label B_ra:
    f = i + 8;
    f[0]:int = (q = (r >> 32L) + f[0]:uint + q * 1262L);
    f = i32_wrap_i64(q >> 32L);
    if (f) goto B_na;
    f = 4;
    goto B_ma;
    label B_pa:
    h = h + 1;
    label B_oa:
    f_ne(h, 40, 1075496);
    unreachable;
    label B_na:
    if (h > 35) goto B_la;
    (i + 12)[0]:int = f;
    f = 5;
    label B_ma:
    i = h + 1;
    g = g + d;
    d = f + h;
    l = select_if(d, l, l < d);
    continue L_ja;
    label B_la:
  }
  f_ne(h + 4, 40, 1075496);
  unreachable;
  label B_ha:
  f_sj(d, 40, d);
  unreachable;
  label B_ga:
  f_dk(a + 4, j, 160);
  a[0] = l;
  label B_fa:
  if (eqz(b & 64)) goto B_sa;
  j = f_bk(c, 0, 160);
  d = a[0];
  if (d < 7) goto B_va;
  if (d >= 41) goto B_ua;
  l = f_jb(j, 1069500, 7, a + 4, d);
  goto B_ta;
  label B_va:
  g = a + 4;
  e = g + (d << 2);
  k = 0;
  l = 0;
  loop L_wa {
    h = k + -1;
    d = 0;
    loop L_xa {
      f = g + d;
      if (f == e) goto B_ta;
      h = h + 1;
      d = d + 4;
      f = f[0]:int;
      if (eqz(f)) continue L_xa;
    }
    if (h > 39) goto B_bb;
    i = select_if(0, i = 40 - h, i > 40);
    if (i == 1) goto B_cb;
    if (i != 2) goto B_db;
    h = h + 2;
    goto B_bb;
    label B_db:
    k = j + (k << 2) + d;
    m = k + 4;
    m[0]:int = (r = (q = i64_extend_i32_u(f)) * 3211403009L + m[0]:uint);
    if (i != 3) goto B_eb;
    h = h + 3;
    goto B_bb;
    label B_eb:
    f = k + 8;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 1849224548L);
    if (i != 4) goto B_fb;
    h = h + 4;
    goto B_bb;
    label B_fb:
    f = k + 12;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 3668416493L);
    if (i != 5) goto B_gb;
    h = h + 5;
    goto B_bb;
    label B_gb:
    f = k + 16;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 3913284084L);
    if (i != 6) goto B_hb;
    h = h + 6;
    goto B_bb;
    label B_hb:
    f = k + 20;
    f[0]:int = (q = (r >> 32L) + f[0]:uint + q * 1593091L);
    f = i32_wrap_i64(q >> 32L);
    if (f) goto B_ab;
    f = 7;
    goto B_za;
    label B_cb:
    h = h + 1;
    label B_bb:
    f_ne(h, 40, 1075496);
    unreachable;
    label B_ab:
    if (h > 32) goto B_ya;
    (k + 24)[0]:int = f;
    f = 8;
    label B_za:
    k = h + 1;
    g = g + d;
    d = f + h;
    l = select_if(d, l, l < d);
    continue L_wa;
    label B_ya:
  }
  f_ne(h + 7, 40, 1075496);
  unreachable;
  label B_ua:
  f_sj(d, 40, d);
  unreachable;
  label B_ta:
  f_dk(a + 4, j, 160);
  a[0] = l;
  label B_sa:
  if (eqz(b & 128)) goto B_ib;
  l = f_bk(c, 0, 160);
  d = a[0];
  if (d < 14) goto B_lb;
  if (d >= 41) goto B_kb;
  j = f_jb(l, 1069528, 14, a + 4, d);
  goto B_jb;
  label B_lb:
  g = a + 4;
  e = g + (d << 2);
  k = 0;
  j = 0;
  loop L_mb {
    h = k + -1;
    d = 0;
    loop L_nb {
      f = g + d;
      if (f == e) goto B_jb;
      h = h + 1;
      d = d + 4;
      f = f[0]:int;
      if (eqz(f)) continue L_nb;
    }
    if (h > 39) goto B_rb;
    i = select_if(0, i = 40 - h, i > 40);
    br_table[B_sb, B_tb, B_tb, ..B_ub](i + -1);
    label B_ub:
    if (i != 4) goto B_vb;
    h = h + 4;
    goto B_rb;
    label B_vb:
    k = l + (k << 2) + d;
    m = k + 12;
    m[0]:int = (r = (q = i64_extend_i32_u(f)) * 781532673L + m[0]:uint);
    if (i != 5) goto B_wb;
    h = h + 5;
    goto B_rb;
    label B_wb:
    f = k + 16;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 64985353L);
    if (i != 6) goto B_xb;
    h = h + 6;
    goto B_rb;
    label B_xb:
    f = k + 20;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 253049085L);
    if (i != 7) goto B_yb;
    h = h + 7;
    goto B_rb;
    label B_yb:
    f = k + 24;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 594863151L);
    if (i != 8) goto B_zb;
    h = h + 8;
    goto B_rb;
    label B_zb:
    f = k + 28;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 3553621484L);
    if (i != 9) goto B_ac;
    h = h + 9;
    goto B_rb;
    label B_ac:
    f = k + 32;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 3288652808L);
    if (i != 10) goto B_bc;
    h = h + 10;
    goto B_rb;
    label B_bc:
    f = k + 36;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 3167596762L);
    if (i != 11) goto B_cc;
    h = h + 11;
    goto B_rb;
    label B_cc:
    f = k + 40;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 2788392729L);
    if (i != 12) goto B_dc;
    h = h + 12;
    goto B_rb;
    label B_dc:
    f = k + 44;
    f[0]:int = (r = (r >> 32L) + f[0]:uint + q * 3911132675L);
    if (i != 13) goto B_ec;
    h = h + 13;
    goto B_rb;
    label B_ec:
    f = k + 48;
    f[0]:int = (q = (r >> 32L) + f[0]:uint + q * 590L);
    f = i32_wrap_i64(q >> 32L);
    if (f) goto B_qb;
    f = 14;
    goto B_pb;
    label B_tb:
    h = select_if(0, d = h + -40, d > h) + 40;
    goto B_rb;
    label B_sb:
    h = h + 1;
    label B_rb:
    f_ne(h, 40, 1075496);
    unreachable;
    label B_qb:
    if (h > 25) goto B_ob;
    (k + 52)[0]:int = f;
    f = 15;
    label B_pb:
    k = h + 1;
    g = g + d;
    d = f + h;
    j = select_if(d, j, j < d);
    continue L_mb;
    label B_ob:
  }
  f_ne(h + 14, 40, 1075496);
  unreachable;
  label B_kb:
  f_sj(d, 40, d);
  unreachable;
  label B_jb:
  f_dk(a + 4, l, 160);
  a[0] = j;
  label B_ib:
  if (eqz(b & 256)) goto B_fc;
  e = 0;
  var n:int = f_bk(c, 0, 160);
  d = a[0];
  if (d < 27) goto B_jc;
  if (d >= 41) goto B_ic;
  o = f_jb(n, 1069584, 27, a + 4, d);
  goto B_gc;
  label B_jc:
  h = a + 4;
  i = h + (d << 2);
  o = 0;
  loop L_kc {
    g = e + 1;
    b = n + (e << 2);
    loop L_lc {
      l = e;
      f = g;
      d = b;
      if (h == i) goto B_gc;
      b = d + 4;
      g = f + 1;
      e = l + 1;
      j = h[0];
      k = h + 4;
      h = k;
      if (eqz(j)) continue L_lc;
    }
    g = 0;
    var p:int = select_if(0, h = 40 - l, h > 40);
    m = select_if(l, 40, l < 40) << 2;
    q = i64_extend_i32_u(j);
    r = 0L;
    h = -160;
    loop L_mc {
      if (m + h) goto B_nc;
      f = f + -1;
      goto B_hc;
      label B_nc:
      d[0]:int = (r = r + d[0]:uint + (b = h + 1069744)[0]:uint * q);
      r = r >> 32L;
      if (b + 4 == 1069692) goto B_oc;
      if ((g | 1) == p) goto B_hc;
      b = d + 4;
      b[0]:int = (r = r + b[0]:uint + (h + 1069748)[0]:uint * q);
      r = r >> 32L;
      d = d + 8;
      f = f + 2;
      h = h + 8;
      g = g + 2;
      continue L_mc;
      label B_oc:
    }
    d = i32_wrap_i64(r);
    if (d) goto B_rc;
    d = 27;
    goto B_qc;
    label B_rc:
    h = l + 27;
    if (h > 39) goto B_pc;
    n[h]:int = d;
    d = 28;
    label B_qc:
    d = d + l;
    o = select_if(d, o, o < d);
    h = k;
    continue L_kc;
    label B_pc:
  }
  f_ne(h, 40, 1075496);
  unreachable;
  label B_ic:
  f_sj(d, 40, d);
  unreachable;
  label B_hc:
  f_ne(f, 40, 1075496);
  unreachable;
  label B_gc:
  f_dk(a + 4, n, 160);
  a[0] = o;
  label B_fc:
  g_a = c + 160;
  return a;
}

function f_e(a:{ a:int, b:int, c:int }):int {
  var e:int_ptr;
  var i:int_ptr;
  var h:int_ptr;
  var g:int_ptr;
  var c:int_ptr;
  var d:int_ptr;
  var f:int;
  var j:int;
  var p:int_ptr;
  var q:int;
  var k:long_ptr@4;
  var b:{ a:int, b:int, c:int } = g_a - 16;
  g_a = b;
  if (a < 245) goto B_c;
  c = f_bl();
  d = 0;
  c = (c - f_nh(c, 8) + f_nh(20, 8) + f_nh(16, 8) + -65544 & -9) + -3;
  if (select_if(c, e = 0 - (f_nh(16, 8) << 2), e > c) <= a) goto B_a;
  c = f_nh(a + 4, 8);
  if (eqz(0[269135]:int)) goto B_b;
  f = 0;
  if (c < 256) goto B_d;
  f = 31;
  if (c > 16777215) goto B_d;
  f = (c >> 6 - (a = clz(c >> 8)) & 1) - (a << 1) + 62;
  label B_d:
  d = 0 - c;
  a = ((f << 2) + 1076808)[0]:int;
  if (eqz(a)) goto B_g;
  g = c << f_fh(f);
  h = 0;
  e = 0;
  loop L_h {
    i = f_ej(f_cl(a));
    if (i < c) goto B_i;
    i = i - c;
    if (i >= d) goto B_i;
    d = i;
    e = a;
    if (i) goto B_i;
    d = 0;
    e = a;
    goto B_f;
    label B_i:
    i = (a + 20)[0]:int;
    h = 
      select_if(select_if(i, h, i != (a = (a + (g >> 29 & 4) + 16)[0]:int)),
                h,
                i);
    g = g << 1;
    if (a) continue L_h;
  }
  if (eqz(h)) goto B_j;
  a = h;
  goto B_f;
  label B_j:
  if (e) goto B_e;
  label B_g:
  e = 0;
  a = f_uh(1 << f) & 0[269135]:int;
  if (eqz(a)) goto B_b;
  a = ((ctz(f_oi(a)) << 2) + 1076808)[0]:int;
  if (eqz(a)) goto B_b;
  label B_f:
  loop L_k {
    e = select_if(a, e, g = (h = f_ej(f_cl(a))) >= c & (h = h - c) < d);
    d = select_if(h, d, g);
    a = f_ch(a);
    if (a) continue L_k;
  }
  if (eqz(e)) goto B_b;
  label B_e:
  a = 0[269234]:int;
  if (a < c) goto B_l;
  if (d >= a - c) goto B_b;
  label B_l:
  a = f_cl(e);
  h = f_kk(a, c);
  f_nc(e);
  if (d < f_nh(16, 8)) goto B_n;
  f_qi(a, c);
  f_gh(h, d);
  if (d < 256) goto B_o;
  f_lc(h, d);
  goto B_m;
  label B_o:
  e = d >> 3;
  d = (e << 3) + 1076544;
  g = 0[269134]:int;
  if (eqz(g & (e = 1 << e))) goto B_q;
  e = d[2];
  goto B_p;
  label B_q:
  0[269134]:int = g | e;
  e = d;
  label B_p:
  d[2] = h;
  e[3] = h;
  h[3] = d;
  h[2] = e;
  goto B_m;
  label B_n:
  f_ug(a, d + c);
  label B_m:
  d = f_mk(a);
  if (eqz(d)) goto B_b;
  goto B_a;
  label B_c:
  c = f_nh(select_if(16, a + 4, f_nh(16, 8) + -5 > a), 8);
  h = 0[269134]:int;
  a = h >> (d = c >> 3);
  if (a & 3) goto B_x;
  if (c <= 0[269234]:int) goto B_b;
  if (a) goto B_w;
  a = 0[269135]:int;
  if (eqz(a)) goto B_b;
  e = ((ctz(f_oi(a)) << 2) + 1076808)[0]:int;
  d = f_ej(f_cl(e)) - c;
  a = f_ch(e);
  if (eqz(a)) goto B_y;
  loop L_z {
    h = f_ej(f_cl(a)) - c;
    d = select_if(h, d, h = h < d);
    e = select_if(a, e, h);
    a = f_ch(a);
    if (a) continue L_z;
  }
  label B_y:
  a = f_cl(e);
  h = f_kk(a, c);
  f_nc(e);
  if (d < f_nh(16, 8)) goto B_s;
  h = f_cl(h);
  f_qi(a, c);
  f_gh(h, d);
  e = 0[269234]:int;
  if (eqz(e)) goto B_t;
  i = e >> 3;
  g = (i << 3) + 1076544;
  e = 0[269236]:int;
  f = 0[269134]:int;
  if (eqz(f & (i = 1 << i))) goto B_v;
  i = g[2];
  goto B_u;
  label B_x:
  c = ((a ^ -1) & 1) + d;
  e = c << 3;
  a = (e + 1076552)[0]:int;
  d = (a + 8)[0]:int;
  if (d == (e = e + 1076544)) goto B_ba;
  d[3] = e;
  e[2] = d;
  goto B_aa;
  label B_ba:
  0[269134]:int = h & -2 << c;
  label B_aa:
  f_ug(a, c << 3);
  d = f_mk(a);
  goto B_a;
  label B_w:
  d = ctz(f_oi(f_uh(1 << (d = d & 31)) & a << d));
  h = d << 3;
  a = (h + 1076552)[0]:int;
  e = (a + 8)[0]:int;
  if (e == (h = h + 1076544)) goto B_da;
  e[3] = h;
  h[2] = e;
  goto B_ca;
  label B_da:
  0[269134]:int = 0[269134]:int & -2 << d;
  label B_ca:
  f_qi(a, c);
  e = f_kk(a, c);
  f_gh(e, h = (d << 3) - c);
  c = 0[269234]:int;
  if (eqz(c)) goto B_ea;
  g = c >> 3;
  d = (g << 3) + 1076544;
  c = 0[269236]:int;
  i = 0[269134]:int;
  if (eqz(i & (g = 1 << g))) goto B_ga;
  g = d[2];
  goto B_fa;
  label B_ga:
  0[269134]:int = i | g;
  g = d;
  label B_fa:
  d[2] = c;
  g[3] = c;
  c[3] = d;
  c[2] = g;
  label B_ea:
  0[269236]:int = e;
  0[269234]:int = h;
  d = f_mk(a);
  goto B_a;
  label B_v:
  0[269134]:int = f | i;
  i = g;
  label B_u:
  g[2] = e;
  i[3] = e;
  e[3] = g;
  e[2] = i;
  label B_t:
  0[269236]:int = h;
  0[269234]:int = d;
  goto B_r;
  label B_s:
  f_ug(a, d + c);
  label B_r:
  d = f_mk(a);
  if (d) goto B_a;
  label B_b:
  d = 0[269234]:int;
  if (d >= c) goto B_pa;
  a = 0[269235]:int;
  if (a > c) goto B_na;
  f_xf(
    b,
    1076536,
    f_nh(c - (a = f_bl()) + f_nh(a, 8) + f_nh(20, 8) + f_nh(16, 8) + 8,
         65536));
  d = b.a;
  if (d) goto B_oa;
  d = 0;
  goto B_a;
  label B_pa:
  a = 0[269236]:int;
  d = d - c;
  if (d >= f_nh(16, 8)) goto B_qa;
  0[269236]:int = 0;
  c = 0[269234]:int;
  0[269234]:int = 0;
  f_ug(a, c);
  d = f_mk(a);
  goto B_a;
  label B_qa:
  e = f_kk(a, c);
  0[269234]:int = d;
  0[269236]:int = e;
  f_gh(e, d);
  f_qi(a, c);
  d = f_mk(a);
  goto B_a;
  label B_oa:
  f = b.c;
  0[269238]:int = (a = 0[269238]:int + (g = b.b));
  0[269239]:int = select_if(e = 0[269239]:int, a, e > a);
  if (eqz(0[269237]:int)) goto B_ta;
  a = 1076960;
  loop L_ua {
    if (d == f_ri(a)) goto B_sa;
    a = a.c;
    if (a) continue L_ua;
    goto B_ra;
  }
  label B_ta:
  a = 0[269245]:int;
  if (eqz(a)) goto B_ma;
  if (d < a) goto B_ma;
  goto B_ia;
  label B_sa:
  if (f_gj(a)) goto B_ra;
  if (f_hj(a) != f) goto B_ra;
  if (f_pg(a, 0[269237]:int)) goto B_la;
  label B_ra:
  0[269245]:int = select_if(a = 0[269245]:int, d, d > a);
  e = d + g;
  a = 1076960;
  loop L_ya {
    if (a.a == e) goto B_xa;
    a = a.c;
    if (a) continue L_ya;
    goto B_wa;
  }
  label B_xa:
  if (f_gj(a)) goto B_wa;
  if (f_hj(a) == f) goto B_va;
  label B_wa:
  e = 0[269237]:int;
  a = 1076960;
  loop L_ab {
    if (a.a > e) goto B_bb;
    if (f_ri(a) > e) goto B_za;
    label B_bb:
    a = a.c;
    if (a) continue L_ab;
  }
  a = 0;
  label B_za:
  h = f_ri(a);
  a = h - (j = f_nh(20, 8)) + -23;
  i = 
    select_if(e, a = a + f_nh(i = f_mk(a), 8) - i, a < e + f_nh(16, 8));
  k = f_mk(i);
  a = f_kk(i, j);
  var l:int = f_bl();
  var m:int = f_nh(l, 8);
  var n:int = f_nh(20, 8);
  var o:int = f_nh(16, 8);
  0[269237]:int = (p = f_kk(d, q = f_nh(p = f_mk(d), 8) - p));
  0[269235]:int = (l = l + g - o + m + n + q);
  p[1] = l | 1;
  m = f_bl();
  n = f_nh(m, 8);
  o = f_nh(20, 8);
  q = f_nh(16, 8);
  f_kk(p, l)[1]:int = q + o + n - m;
  0[269244]:int = 2097152;
  f_qi(i, j);
  var r:long = 0[269240]:long@4;
  (k + 8)[0]:long@4 = 0[269242]:long@4;
  k[0] = r;
  0[269243]:int = f;
  0[269241]:int = g;
  0[269240]:int = d;
  0[269242]:int = k;
  loop L_cb {
    d = f_kk(a, 4);
    a.b = f_al();
    a = d;
    if (h > d + 4) continue L_cb;
  }
  if (i == e) goto B_ha;
  a = i - e;
  f_sg(e, a, f_kk(e, a));
  if (a < 256) goto B_db;
  f_lc(e, a);
  goto B_ha;
  label B_db:
  d = a >> 3;
  a = (d << 3) + 1076544;
  h = 0[269134]:int;
  if (eqz(h & (d = 1 << d))) goto B_fb;
  d = a.c;
  goto B_eb;
  label B_fb:
  0[269134]:int = h | d;
  d = a;
  label B_eb:
  a.c = e;
  d[3] = e;
  e[3] = a;
  e[2] = d;
  goto B_ha;
  label B_va:
  h = a.a;
  a.a = d;
  a.b = a.b + g;
  a = f_mk(d);
  e = f_nh(a, 8);
  g = f_mk(h);
  i = f_nh(g, 8);
  d = d + e - a;
  e = f_kk(d, c);
  f_qi(d, c);
  a = h + i - g;
  c = a - c + d;
  if (0[269237]:int == a) goto B_gb;
  if (0[269236]:int == a) goto B_ka;
  if (f_gi(a)) goto B_ja;
  h = f_ej(a);
  if (h < 256) goto B_ib;
  f_nc(a);
  goto B_hb;
  label B_ib:
  g = (a + 12)[0]:int;
  if (g == (i = (a + 8)[0]:int)) goto B_jb;
  i[3] = g;
  g[2] = i;
  goto B_hb;
  label B_jb:
  0[269134]:int = 0[269134]:int & -2 << (h >> 3);
  label B_hb:
  c = h + c;
  a = f_kk(a, h);
  goto B_ja;
  label B_gb:
  0[269237]:int = e;
  0[269235]:int = (a = 0[269235]:int + c);
  e[1] = a | 1;
  d = f_mk(d);
  goto B_a;
  label B_na:
  0[269235]:int = (d = a - c);
  0[269237]:int = (e = f_kk(a = 0[269237]:int, c));
  e[1] = d | 1;
  f_qi(a, c);
  d = f_mk(a);
  goto B_a;
  label B_ma:
  0[269245]:int = d;
  goto B_ia;
  label B_la:
  a.b = a.b + g;
  f_ge(0[269237]:int, 0[269235]:int + g);
  goto B_ha;
  label B_ka:
  0[269236]:int = e;
  0[269234]:int = (a = 0[269234]:int + c);
  f_gh(e, a);
  d = f_mk(d);
  goto B_a;
  label B_ja:
  f_sg(e, c, a);
  if (c < 256) goto B_kb;
  f_lc(e, c);
  d = f_mk(d);
  goto B_a;
  label B_kb:
  c = c >> 3;
  a = (c << 3) + 1076544;
  h = 0[269134]:int;
  if (eqz(h & (c = 1 << c))) goto B_mb;
  c = a.c;
  goto B_lb;
  label B_mb:
  0[269134]:int = h | c;
  c = a;
  label B_lb:
  a.c = e;
  c[3] = e;
  e[3] = a;
  e[2] = c;
  d = f_mk(d);
  goto B_a;
  label B_ia:
  0[269246]:int = 4095;
  0[269243]:int = f;
  0[269241]:int = g;
  0[269240]:int = d;
  0[269139]:int = 1076544;
  0[269141]:int = 1076552;
  0[269138]:int = 1076544;
  0[269143]:int = 1076560;
  0[269140]:int = 1076552;
  0[269145]:int = 1076568;
  0[269142]:int = 1076560;
  0[269147]:int = 1076576;
  0[269144]:int = 1076568;
  0[269149]:int = 1076584;
  0[269146]:int = 1076576;
  0[269151]:int = 1076592;
  0[269148]:int = 1076584;
  0[269153]:int = 1076600;
  0[269150]:int = 1076592;
  0[269155]:int = 1076608;
  0[269152]:int = 1076600;
  0[269154]:int = 1076608;
  0[269157]:int = 1076616;
  0[269156]:int = 1076616;
  0[269159]:int = 1076624;
  0[269158]:int = 1076624;
  0[269161]:int = 1076632;
  0[269160]:int = 1076632;
  0[269163]:int = 1076640;
  0[269162]:int = 1076640;
  0[269165]:int = 1076648;
  0[269164]:int = 1076648;
  0[269167]:int = 1076656;
  0[269166]:int = 1076656;
  0[269169]:int = 1076664;
  0[269168]:int = 1076664;
  0[269171]:int = 1076672;
  0[269173]:int = 1076680;
  0[269170]:int = 1076672;
  0[269175]:int = 1076688;
  0[269172]:int = 1076680;
  0[269177]:int = 1076696;
  0[269174]:int = 1076688;
  0[269179]:int = 1076704;
  0[269176]:int = 1076696;
  0[269181]:int = 1076712;
  0[269178]:int = 1076704;
  0[269183]:int = 1076720;
  0[269180]:int = 1076712;
  0[269185]:int = 1076728;
  0[269182]:int = 1076720;
  0[269187]:int = 1076736;
  0[269184]:int = 1076728;
  0[269189]:int = 1076744;
  0[269186]:int = 1076736;
  0[269191]:int = 1076752;
  0[269188]:int = 1076744;
  0[269193]:int = 1076760;
  0[269190]:int = 1076752;
  0[269195]:int = 1076768;
  0[269192]:int = 1076760;
  0[269197]:int = 1076776;
  0[269194]:int = 1076768;
  0[269199]:int = 1076784;
  0[269196]:int = 1076776;
  0[269201]:int = 1076792;
  0[269198]:int = 1076784;
  0[269200]:int = 1076792;
  e = f_bl();
  h = f_nh(e, 8);
  i = f_nh(20, 8);
  f = f_nh(16, 8);
  0[269237]:int = (a = f_kk(d, k = f_nh(a = f_mk(d), 8) - a));
  0[269235]:int = (d = e + g - f + h + i + k);
  a.b = d | 1;
  e = f_bl();
  h = f_nh(e, 8);
  g = f_nh(20, 8);
  i = f_nh(16, 8);
  f_kk(a, d)[1]:int = i + g + h - e;
  0[269244]:int = 2097152;
  label B_ha:
  d = 0;
  a = 0[269235]:int;
  if (a <= c) goto B_a;
  0[269235]:int = (d = a - c);
  0[269237]:int = (e = f_kk(a = 0[269237]:int, c));
  e[1] = d | 1;
  f_qi(a, c);
  d = f_mk(a);
  label B_a:
  g_a = b + 16;
  return d;
}

function f_f(a:int, b:int, c:int, d:int) {
  var m:int;
  var n:int;
  var g:int;
  var i:int;
  var h:int;
  var l:int;
  var k:long_ptr@1;
  var o:long_ptr;
  var j:int;
  var s:int;
  var w:long_ptr;
  var u:int;
  var t:int;
  var e:int = g_a - 160;
  g_a = e;
  var f:int = b[2]:int;
  g = b[1]:int;
  h = g[201]:ushort;
  if (h < 11) goto B_o;
  i = b[0]:int;
  f_cf(e + 136, f);
  j = e + 136 + 8;
  h = j[0]:int;
  k = e[35]:int;
  b = e[34]:int;
  l = f_wh(408, 8);
  if (eqz(l)) goto B_i;
  l[66]:int = 0;
  l[201]:short = (f = (m = g[201]:ushort) + (b ^ -1));
  (e + 64 + 8)[0]:int = ((n = g + b * 12) + 276)[0]:int;
  e[8]:long = (n + 268)[0]:long@4;
  j[0]:long = ((n = g + b * 24) + 9)[0]:long@1;
  (e + 151)[0]:long@1 = (n + 16)[0]:long@1;
  e[17]:long = n[1]:long@1;
  if (f >= 12) goto B_h;
  if (m - (j = b + 1) != f) goto B_g;
  m = n[0]:ubyte;
  f_dk(l + 268, g + j * 12 + 268, f * 12);
  j = f_dk(l, g + j * 24, f * 24);
  g[201]:short = b;
  (e + 8)[0]:int = (e + 64 + 8)[0]:int;
  (e + 80 + 8)[0]:long = (e + 136 + 8)[0]:long;
  (e + 80 + 15)[0]:long@1 = (e + 136 + 15)[0]:long@1;
  e[0]:long = e[8]:long;
  e[10]:long = e[17]:long;
  b = select_if(j, g, k);
  f = b + h * 12 + 268;
  k = h + 1;
  if (k <= (n = b[201]:ushort)) goto B_n;
  f[0]:long@4 = c[0]:long@4;
  (f + 8)[0]:int = (c + 8)[0]:int;
  goto B_m;
  label B_o:
  b = g + f * 12 + 268;
  i = f + 1;
  if (i <= h) goto B_q;
  b[0]:long@4 = c[0]:long@4;
  (b + 8)[0]:int = (c + 8)[0]:int;
  goto B_p;
  label B_q:
  f_ek(g + i * 12 + 268, b, (l = h - f) * 12);
  (b + 8)[0]:int = (c + 8)[0]:int;
  b[0]:long@4 = c[0]:long@4;
  f_ek(g + i * 24, g + f * 24, l * 24);
  label B_p:
  o = g + f * 24;
  (o + 16)[0]:long = (d + 16)[0]:long;
  o[0] = d[0]:long;
  (o + 8)[0]:long = (d + 8)[0]:long;
  g[201]:short = h + 1;
  goto B_l;
  label B_n:
  f_ek(b + k * 12 + 268, f, (o = n - h) * 12);
  (f + 8)[0]:int = (c + 8)[0]:int;
  f[0]:long@4 = c[0]:long@4;
  f_ek(b + k * 24, b + h * 24, o * 24);
  label B_m:
  o = b + h * 24;
  (o + 16)[0]:long = (d + 16)[0]:long;
  o[0] = d[0]:long;
  f = e + 104 + 8;
  f[0]:long = (c = e + 8)[0]:long;
  h = e + 40 + 8;
  h[0]:long = (e + 80 + 8)[0]:long;
  k = e + 40 + 15;
  k[0] = (e + 80 + 15)[0]:long@1;
  (o + 8)[0]:long = (d + 8)[0]:long;
  b[201]:short = n + 1;
  e[13]:long = e[0]:long;
  e[5]:long = e[10]:long;
  if ((m & 255) != 6) goto B_k;
  label B_l:
  a[14]:int = o;
  a[16]:byte = 6;
  goto B_j;
  label B_k:
  (e + 24 + 8)[0]:long = f[0]:long;
  c[0]:long = h[0]:long;
  (e + 15)[0]:long@1 = k[0];
  e[3]:long = e[13]:long;
  e[0]:long = e[5]:long;
  d = g[66]:int;
  if (d) goto B_s;
  n = 0;
  goto B_r;
  label B_s:
  k = e + 136 + 15;
  n = i;
  c = g;
  b = 0;
  var p:int = j;
  var q:int = m;
  loop L_t {
    g = d;
    if (n != b) goto B_f;
    d = c[200]:ushort;
    b = g[201]:ushort;
    if (b < 11) goto B_y;
    f_cf(e + 136, d);
    i = e[36]:int;
    var r:int = e[35]:int;
    d = e[34]:int;
    s = g[201]:ushort;
    l = f_wh(456, 8);
    if (eqz(l)) goto B_e;
    l[66]:int = 0;
    l[201]:short = (b = (j = g[201]:ushort) + (d ^ -1));
    t = e + 120 + 8;
    t[0]:int = ((f = g + d * 12) + 276)[0]:int;
    e[15]:long = (f + 268)[0]:long@4;
    h = e + 136 + 8;
    h[0]:long = ((f = g + d * 24) + 9)[0]:long@1;
    k[0] = (f + 16)[0]:long@1;
    e[17]:long = f[1]:long@1;
    if (b >= 12) goto B_d;
    if (j - (c = d + 1) != b) goto B_c;
    m = f[0]:ubyte;
    f_dk(l + 268, g + c * 12 + 268, b * 12);
    f = f_dk(l, g + c * 24, b * 24);
    g[201]:short = d;
    u = e + 104 + 8;
    u[0]:int = t[0]:int;
    t = e + 80 + 8;
    t[0]:long = h[0]:long;
    var v:long_ptr@1 = e + 80 + 15;
    v[0] = k[0];
    e[13]:long = e[15]:long;
    e[10]:long = e[17]:long;
    b = f[201]:ushort;
    j = b + 1;
    if (b >= 12) goto B_b;
    d = s - d;
    if (d != j) goto B_a;
    n = n + 1;
    f_dk(f + 408, g + (c << 2) + 408, d << 2);
    d = 0;
    loop L_aa {
      c = (f + (d << 2) + 408)[0]:int;
      c[200]:short = d;
      c[66]:int = f;
      if (d >= b) goto B_z;
      d = d + (d < b);
      if (d <= b) continue L_aa;
    }
    label B_z:
    w = e + 64 + 8;
    w[0] = u[0]:long;
    u = e + 40 + 8;
    u[0]:long = t[0]:long;
    t = e + 40 + 15;
    t[0]:long@1 = v[0];
    e[8]:long = e[13]:long;
    e[5]:long = e[10]:long;
    d = select_if(f, g, r);
    b = d + i * 12 + 268;
    c = i + 1;
    if (c <= (j = d[201]:ushort)) goto B_x;
    b[0]:long@4 = e[3]:long;
    (b + 8)[0]:int = (e + 24 + 8)[0]:int;
    k[0] = (e + 15)[0]:long@1;
    h[0]:long = (e + 8)[0]:long;
    e[17]:long = e[0]:long;
    goto B_w;
    label B_y:
    f = d + 1;
    h = b + 1;
    c = g + d * 12 + 268;
    if (b > d) goto B_ca;
    c[0]:long@4 = e[3]:long;
    (c + 8)[0]:int = (e + 24 + 8)[0]:int;
    c = g + d * 24;
    c[0]:byte = q;
    c[1]:long@1 = e[0]:long;
    (c + 9)[0]:long@1 = (e + 8)[0]:long;
    (c + 16)[0]:long@1 = (e + 15)[0]:long@1;
    c = g + 408;
    goto B_ba;
    label B_ca:
    f_ek(g + f * 12 + 268, c, (i = b - d) * 12);
    (c + 8)[0]:int = (e + 24 + 8)[0]:int;
    c[0]:long@4 = e[3]:long;
    f_ek(g + f * 24, c = g + d * 24, i * 24);
    c[0]:byte = q;
    c[1]:long@1 = e[0]:long;
    (c + 9)[0]:long@1 = (e + 8)[0]:long;
    (c + 16)[0]:long@1 = (e + 15)[0]:long@1;
    f_ek((d << 2) + (c = g + 408) + 8, c + (f << 2), i << 2);
    label B_ba:
    c[f]:int = p;
    g[201]:short = h;
    if (f >= b + 2) goto B_v;
    i = b - d;
    c = i + 1 & 3;
    if (eqz(c)) goto B_da;
    d = g + (d << 2) + 412;
    loop L_ea {
      h = d[0]:int;
      h[200]:short = f;
      h[66]:int = g;
      d = d + 4;
      f = f + 1;
      c = c + -1;
      if (c) continue L_ea;
    }
    label B_da:
    if (i < 3) goto B_v;
    d = f + 3;
    c = -2 - b;
    b = (f << 2) + g + 420;
    loop L_fa {
      f = (b + -12)[0]:int;
      f[200]:short = d + -3;
      f[66]:int = g;
      f = (b + -8)[0]:int;
      f[200]:short = d + -2;
      f[66]:int = g;
      f = (b + -4)[0]:int;
      f[200]:short = d + -1;
      f[66]:int = g;
      f = b[0]:int;
      f[200]:short = d;
      f[66]:int = g;
      b = b + 16;
      if (c + (d = d + 4) != 3) continue L_fa;
      goto B_v;
    }
    label B_x:
    f_ek(d + c * 12 + 268, b, (s = j - i) * 12);
    (b + 8)[0]:int = (e + 24 + 8)[0]:int;
    b[0]:long@4 = e[3]:long;
    k[0] = (e + 15)[0]:long@1;
    h[0]:long = (e + 8)[0]:long;
    e[17]:long = e[0]:long;
    f_ek(d + c * 24, d + i * 24, s * 24);
    label B_w:
    s = j + 1;
    b = d + i * 24;
    b[0]:byte = q;
    b[1]:long@1 = e[17]:long;
    (b + 9)[0]:long@1 = h[0]:long;
    (b + 16)[0]:long@1 = k[0];
    b = d + 408;
    q = i + 2;
    if (q >= (h = j + 2)) goto B_ga;
    f_ek(b + (q << 2), b + (c << 2), j - i << 2);
    label B_ga:
    b[c]:int = p;
    d[201]:short = s;
    if (c >= h) goto B_ha;
    p = j - i;
    h = p + 1 & 3;
    if (eqz(h)) goto B_ia;
    b = d + (i << 2) + 412;
    loop L_ja {
      i = b[0]:int;
      i[200]:short = c;
      i[66]:int = d;
      b = b + 4;
      c = c + 1;
      h = h + -1;
      if (h) continue L_ja;
    }
    label B_ia:
    if (p < 3) goto B_ha;
    b = c + 3;
    i = -2 - j;
    c = d + (c << 2) + 420;
    loop L_ka {
      h = (c + -12)[0]:int;
      h[200]:short = b + -3;
      h[66]:int = d;
      h = (c + -8)[0]:int;
      h[200]:short = b + -2;
      h[66]:int = d;
      h = (c + -4)[0]:int;
      h[200]:short = b + -1;
      h[66]:int = d;
      h = c[0]:int;
      h[200]:short = b;
      h[66]:int = d;
      c = c + 16;
      if (i + (b = b + 4) != 3) continue L_ka;
    }
    label B_ha:
    if ((m & 255) != 6) goto B_u;
    label B_v:
    a[14]:int = o;
    a[16]:byte = 6;
    goto B_j;
    label B_u:
    (e + 24 + 8)[0]:long = w[0];
    (e + 8)[0]:long = u[0]:long;
    (e + 15)[0]:long@1 = t[0]:long@1;
    e[3]:long = e[8]:long;
    e[0]:long = e[5]:long;
    c = g;
    b = n;
    p = f;
    q = m;
    i = n;
    d = g[66]:int;
    if (d) continue L_t;
  }
  label B_r:
  a[0]:long = e[3]:long;
  a[17]:long@1 = e[0]:long;
  a[14]:int = o;
  a[16]:byte = m;
  (a + 52)[0]:int = l;
  (a + 48)[0]:int = n;
  (a + 44)[0]:int = g;
  (a + 40)[0]:int = i;
  (a + 8)[0]:long = (e + 24 + 8)[0]:long;
  (a + 25)[0]:long@1 = (e + 8)[0]:long;
  (a + 32)[0]:long@1 = (e + 15)[0]:long@1;
  label B_j:
  g_a = e + 160;
  return ;
  label B_i:
  f_mj(408, 8);
  unreachable;
  label B_h:
  f_sj(f, 11, 1048880);
  unreachable;
  label B_g:
  f_rf(1048824, 40, 1048864);
  unreachable;
  label B_f:
  f_rf(1048912, 53, 1048968);
  unreachable;
  label B_e:
  f_mj(456, 8);
  unreachable;
  label B_d:
  f_sj(b, 11, 1048880);
  unreachable;
  label B_c:
  f_rf(1048824, 40, 1048864);
  unreachable;
  label B_b:
  f_sj(j, 12, 1048896);
  unreachable;
  label B_a:
  f_rf(1048824, 40, 1048864);
  unreachable;
}

function f_g(a:{ a:int, b:int }, b:{ a:long, b:long, c:long, d:ushort }, c:int, d:int) {
  var f:int;
  var g:int;
  var q:long;
  var r:long;
  var s:long;
  var v:long;
  var w:long;
  var ca:long;
  var ea:long;
  var fa:long;
  var ga:long;
  var ha:long;
  var h:int;
  var l:int;
  var t:long;
  var o:long;
  var x:long;
  var p:long;
  var z:long;
  var m:long;
  var u:long;
  var y:long;
  var da:long;
  var n:long;
  var ba:long;
  var k:byte_ptr;
  var aa:long;
  var i:int;
  var e:{ a:long, b:short, c:long, d:int } = g_a - 48;
  g_a = e;
  m = b.a;
  if (eqz(m)) goto B_e;
  n = b.b;
  if (eqz(n)) goto B_f;
  o = b.c;
  if (eqz(o)) goto B_g;
  o = m + o;
  if (o < m) goto B_h;
  p = m - n;
  if (p > m) goto B_i;
  if (d < 17) goto B_j;
  if (o > 2305843009213693951L) goto B_o;
  e.b = (b = b.d);
  e.a = p;
  g = 
    (b -
     (f = 
        select_if(
          (g = 
             select_if(
               (g = 
                  select_if(
                    (g = select_if((g = select_if(b + -32, b, f = o < 4294967296L)) + -16,
                                   g,
                                   f = (o = select_if(o << 32L, o, f)) < 281474976710656L)) +
                    -8,
                    g,
                    f = (o = select_if(o << 16L, o, f)) < 72057594037927936L)) +
               -4,
               g,
               f = (o = select_if(o << 8L, o, f)) < 1152921504606846976L)) +
          -2,
          g,
          f = (o = select_if(o << 4L, o, f)) < 4611686018427387904L) +
        (i32_wrap_i64((q = select_if(o << 2L, o, f)) >> 63L) ^ -1)) <<
     16) >>
    16;
  if (g < 0) goto B_k;
  e.c = (o = -1L >> (r = i64_extend_i32_u(g))) & p;
  if (p > o) goto B_l;
  e.b = b;
  e.a = m;
  e.c = o & m;
  if (m > o) goto B_m;
  b = (((-96 - f << 16) >> 16) * 80 + 86960) / 2126;
  if (b >= 81) goto B_n;
  b = b << 4;
  o = (b + 1070232)[0]:long;
  n = o & 4294967295L;
  t = n * (s = (m = m << (r = r & 63L)) >> 32L);
  u = t >> 32L;
  x = 
    u + (v = o >> 32L) * s + (w = (o = v * (m = m & 4294967295L)) >> 32L);
  y = 
    (t & 4294967295L) + (n * m >> 32L) + (o & 4294967295L) + 2147483648L >>
    32L;
  z = 
    1L << (o = i64_extend_i32_u(0 - f + (b + 1070240)[0]:ushort & 63));
  t = z + -1L;
  r = n * (p = (m = p << r) >> 32L);
  aa = (r & 4294967295L) + (n * (m = m & 4294967295L) >> 32L) +
       ((m = v * m) & 4294967295L) +
       2147483648L >>
       32L;
  p = v * p;
  ba = m >> 32L;
  r = r >> 32L;
  b = (b + 1070242)[0]:ushort;
  da = v * (ca = (m = q << ((q ^ -1L) >> 63L)) >> 32L);
  fa = 
    da + (ea = (q = n * ca) >> 32L) +
    (ga = (fa = v * (m = m & 4294967295L)) >> 32L) +
    (ha = 
       (q & 4294967295L) + (n * m >> 32L) + (fa & 4294967295L) + 2147483648L >>
       32L) +
    1L;
  g = i32_wrap_i64(fa >> o);
  if (g < 10000) goto B_s;
  if (g < 1000000) goto B_r;
  if (g < 100000000) goto B_q;
  h = select_if(8, 9, f = g < 1000000000);
  f = select_if(100000000, 1000000000, f);
  goto B_p;
  label B_s:
  if (g < 100) goto B_t;
  h = select_if(2, 3, f = g < 1000);
  f = select_if(100, 1000, f);
  goto B_p;
  label B_t:
  f = select_if(1, 10, g < 10);
  h = g > 9;
  goto B_p;
  label B_r:
  h = select_if(4, 5, f = g < 100000);
  f = select_if(10000, 100000, f);
  goto B_p;
  label B_q:
  h = select_if(6, 7, f = g < 10000000);
  f = select_if(1000000, 10000000, f);
  label B_p:
  x = x + y;
  m = fa & t;
  i = h - b + 1;
  r = fa - (ba = p + r + ba + aa) + 1L;
  p = r & t;
  b = 0;
  loop L_u {
    var j:byte_ptr = g / f;
    if (d == b) goto B_y;
    k = c + b;
    k[0] = (l = j + 48);
    if (
      r > (q = (n = i64_extend_i32_u(g = g - j * f) << o) + m)) goto B_d;
    if (h != b) goto B_v;
    b = b + 1;
    g = select_if(b, d, b > d);
    n = 1L;
    loop L_z {
      q = n;
      r = p;
      if (g == b) goto B_x;
      n = q * 10L;
      (c + b)[0]:byte = (f = i32_wrap_i64((m = m * 10L) >> o) + 48);
      b = b + 1;
      p = r * 10L;
      if (p <= (m = m & t)) continue L_z;
    }
    if (b + -1 >= d) goto B_w;
    v = p - m;
    g = v >= z;
    o = n * (fa - x);
    y = o + n;
    t = o - n;
    if (t <= m) goto B_c;
    if (v < z) goto B_c;
    j = c + b + -1;
    v = r * 10L - z + m;
    fa = z - t;
    s = t - m;
    o = 0L;
    loop L_aa {
      n = m + z;
      if (n < t) goto B_ba;
      if (s + o >= fa + m) goto B_ba;
      g = 1;
      goto B_c;
      label B_ba:
      j[0] = (f = f + -1);
      r = v + o;
      g = r >= z;
      if (n >= t) goto B_b;
      o = o - z;
      m = n;
      if (r >= z) continue L_aa;
      goto B_b;
    }
    label B_y:
    f_ne(d, d, 1071804);
    unreachable;
    label B_x:
    f_ne(g, d, 1071820);
    unreachable;
    label B_w:
    f_sj(b, d, b);
    unreachable;
    label B_v:
    b = b + 1;
    j = f < 10;
    f = f / 10;
    if (eqz(j)) continue L_u;
  }
  f_rf(1071776, 25, 1071752);
  unreachable;
  label B_o:
  f_rf(1071688, 45, 1071736);
  unreachable;
  label B_n:
  f_ne(b, 81, 1071576);
  unreachable;
  label B_m:
  e.d = 0;
  f_qe(e + 16, e, e + 24);
  unreachable;
  label B_l:
  e.d = 0;
  f_qe(e + 16, e, e + 24);
  unreachable;
  label B_k:
  f_rf(1069300, 29, 1069364);
  unreachable;
  label B_j:
  f_rf(1070020, 45, 1071672);
  unreachable;
  label B_i:
  f_rf(1069948, 55, 1071656);
  unreachable;
  label B_h:
  f_rf(1069876, 54, 1071640);
  unreachable;
  label B_g:
  f_rf(1069832, 28, 1071624);
  unreachable;
  label B_f:
  f_rf(1069784, 29, 1071608);
  unreachable;
  label B_e:
  f_rf(1069739, 28, 1071592);
  unreachable;
  label B_d:
  g = b + 1;
  if (b >= d) goto B_da;
  t = r - q;
  b = t >= (o = i64_extend_i32_u(f) << o);
  p = fa - x;
  aa = p + 1L;
  z = p + -1L;
  if (z <= q) goto B_ca;
  if (t < o) goto B_ca;
  q = m + o;
  t = q + u + w + y + v * (s - ca) - ea - ga - ha;
  p = ea + ga + ha + da;
  y = 0L - x + n + m;
  s = 2L - ba + q + n;
  loop L_ea {
    v = n + q;
    if (v < z) goto B_fa;
    if (y + p >= n + t) goto B_fa;
    q = n + m;
    b = 1;
    goto B_ca;
    label B_fa:
    k[0] = (l = l + -1);
    m = m + o;
    fa = s + p;
    if (v >= z) goto B_ga;
    q = q + o;
    t = t + o;
    p = p - o;
    if (fa >= o) continue L_ea;
    label B_ga:
  }
  b = fa >= o;
  q = n + m;
  goto B_ca;
  label B_da:
  f_sj(g, d, b);
  unreachable;
  label B_ca:
  if (aa <= q) goto B_ja;
  if (eqz(b)) goto B_ja;
  m = q + o;
  if (m < aa) goto B_ia;
  if (aa - q >= m - aa) goto B_ia;
  label B_ja:
  if (q < 2L) goto B_ka;
  if (q <= r + -4L) goto B_ha;
  label B_ka:
  a.a = 0;
  goto B_a;
  label B_ia:
  a.a = 0;
  goto B_a;
  label B_ha:
  a.b = g;
  a.a = c;
  (a + 8)[0]:short = i;
  goto B_a;
  label B_c:
  n = m;
  label B_b:
  if (y <= n) goto B_na;
  if (eqz(g)) goto B_na;
  m = n + z;
  if (m < y) goto B_ma;
  if (y - n >= m - y) goto B_ma;
  label B_na:
  if (q * 20L > n) goto B_oa;
  if (n <= q * -40L + p) goto B_la;
  label B_oa:
  a.a = 0;
  goto B_a;
  label B_ma:
  a.a = 0;
  goto B_a;
  label B_la:
  a.b = b;
  a.a = c;
  (a + 8)[0]:short = i;
  label B_a:
  g_a = e + 48;
}

function f_h(a:{ a:int, b:int }, b:int) {
  var f:int_ptr;
  var h:int;
  var i:int;
  var e:int;
  var p:long;
  var o:long;
  var g:int;
  var d:int_ptr;
  var l:int_ptr;
  var k:int;
  var j:int;
  var m:int;
  var n:int;
  var c:int = g_a - 128;
  g_a = c;
  d = b + 8;
  e = d[0];
  if (e >= (f = (b + 4)[0]:int)) goto B_d;
  g = b[0]:int;
  loop L_e {
    h = (g + e)[0]:ubyte;
    i = h + -9;
    if (i > 23) goto B_c;
    if (eqz(1 << i & 8388627)) goto B_c;
    d[0] = (e = e + 1);
    if (f != e) continue L_e;
  }
  label B_d:
  c[8]:int = 5;
  a.b = f_df(b, c + 32);
  goto B_b;
  label B_c:
  if (h != 123) goto B_j;
  b[24]:byte = (i = b[24]:ubyte + -1);
  if (i & 255) goto B_k;
  c[8]:int = 21;
  a.b = f_df(b, c + 32);
  goto B_b;
  label B_k:
  (b + 8)[0]:int = (e = e + 1);
  if (eqz(0[134564]:long)) goto B_m;
  o = 0[134566]:long;
  p = 0[134565]:long;
  i = 1076520;
  goto B_l;
  label B_m:
  f_di(c);
  0[134564]:long = 1L;
  0[134566]:long = (o = c[1]:long);
  i = 1076520;
  p = c[0]:long;
  label B_l:
  i[0]:long = p + 1L;
  f_gb(c + 80, 20, 8, 0);
  (c + 56)[0]:long = (c + 80 + 12)[0]:long@4;
  c[5]:long = o;
  c[4]:long = p;
  c[6]:long = c[21]:long@4;
  if (e >= f) goto B_i;
  j = b + 12;
  k = c + 80 | 4;
  i = b + 8;
  d = 0;
  loop L_n {
    h = (g + e)[0]:ubyte;
    br_table[B_x, B_x, B_v, B_v, B_x, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_x, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_v, B_w, ..B_y](
      h + -9);
    label B_y:
    if (h != 125) goto B_v;
    (c + 16 + 8)[0]:long = (c + 32 + 8)[0]:long;
    c[2]:long = c[4]:long;
    l = (c + 60)[0]:int;
    j = c[12]:int;
    k = c[13]:int;
    m = c[14]:int;
    d = 1;
    goto B_g;
    label B_x:
    i[0]:int = (e = e + 1);
    if (f != e) continue L_n;
    goto B_i;
    label B_w:
    if (eqz(d & 1)) goto B_t;
    i[0]:int = (e = e + 1);
    if (e >= f) goto B_z;
    loop L_aa {
      h = (g + e)[0]:ubyte;
      d = h + -9;
      if (d > 23) goto B_u;
      if (eqz(1 << d & 8388627)) goto B_u;
      i[0]:int = (e = e + 1);
      if (f != e) continue L_aa;
    }
    label B_z:
    c[20]:int = 5;
    n = f_df(b, c + 80);
    goto B_h;
    label B_v:
    if (eqz(d & 1)) goto B_u;
    c[20]:int = 8;
    n = f_df(b, c + 80);
    goto B_h;
    label B_u:
    if (h == 34) goto B_s;
    if (h == 125) goto B_r;
    label B_t:
    c[20]:int = 16;
    n = f_df(b, c + 80);
    goto B_h;
    label B_s:
    b[5]:int = 0;
    i[0]:int = i[0]:int + 1;
    f_r(c + 104, b, j);
    if (c[26]:int) goto B_p;
    h = c[29]:int;
    d = c[28]:int;
    if (eqz(c[27]:int)) goto B_q;
    (c + 88)[0]:int = h;
    c[21]:int = d;
    c[80]:byte = 5;
    n = f_od(c + 80, c + 120, 1049316);
    goto B_h;
    label B_r:
    c[20]:int = 18;
    n = f_df(b, c + 80);
    goto B_h;
    label B_q:
    e = i[0]:int;
    if (e >= (f = (l = b + 4)[0])) goto B_da;
    g = b[0]:int;
    loop L_ea {
      br_table[B_fa, B_fa, B_ca, B_ca, B_fa, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_fa, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ca, B_ba, ..B_ca](
        (g + e)[0]:ubyte + -9)
      label B_fa:
      i[0]:int = (e = e + 1);
      if (f != e) continue L_ea;
    }
    label B_da:
    c[26]:int = 3;
    n = f_df(b, c + 104);
    goto B_h;
    label B_ca:
    c[26]:int = 6;
    n = f_df(b, c + 104);
    goto B_h;
    label B_ba:
    i[0]:int = e + 1;
    f_ac(c + 80, b);
    if (eqz(c[20]:int)) goto B_o;
    n = c[21]:int;
    goto B_h;
    label B_p:
    n = c[27]:int;
    goto B_h;
    label B_o:
    (c + 64 + 8)[0]:int = (e = (k + 8)[0]:int);
    c[8]:long = (p = k[0]:long@4);
    (c + 80 + 8)[0]:int = e;
    c[10]:long = p;
    f_lb(c + 104, c + 32, d, h, c + 80);
    e = c[26]:int;
    if (eqz(e)) goto B_ga;
    g = c[27]:int;
    if (eqz(g)) goto B_ga;
    f_mi(e, g, 1);
    label B_ga:
    e = i[0]:int;
    if (e >= (f = l[0])) goto B_i;
    g = b[0]:int;
    d = 1;
    continue L_n;
  }
  label B_j:
  n = f(b, c + 120, 1049096);
  goto B_f;
  label B_i:
  c[20]:int = 3;
  n = f_df(b, c + 80);
  label B_h:
  d = c[12]:int;
  if (eqz(d)) goto B_ha;
  if ((c + 60)[0]:int) goto B_ja;
  k = d + 1;
  goto B_ia;
  label B_ja:
  i = c[13]:int;
  e = i + 8;
  g = i + (k = d + 1);
  o = (i[0]:long ^ -1L) & -9187201950435737472L;
  loop L_ka {
    if (o == 0L) goto B_ma;
    p = o;
    goto B_la;
    label B_ma:
    loop L_na {
      if (e >= g) goto B_ia;
      i = i + -160;
      p = e[0]:long;
      f = e + 8;
      e = f;
      p = p & -9187201950435737472L;
      if (p == -9187201950435737472L) continue L_na;
    }
    p = p ^ -9187201950435737472L;
    e = f;
    label B_la:
    o = p + -1L & p;
    f = i + (0 - (i32_wrap_i64(ctz(p)) >> 3)) * 20;
    h = (f + -8)[0]:int;
    if (eqz(h)) continue L_ka;
    f_mi((f + -12)[0]:int, h, 1);
    continue L_ka;
  }
  label B_ia:
  i = d + (e = i32_wrap_i64(i64_extend_i32_u(k) * 20L) + 7 & -8) + 9;
  if (eqz(i)) goto B_ha;
  f_mi(c[13]:int - e, i, 8);
  label B_ha:
  d = 0;
  label B_g:
  b[24]:byte = b[24]:ubyte + 1;
  f = b + 8;
  e = f[0];
  if (e >= (h = (b + 4)[0]:int)) goto B_xa;
  i = b[0]:int;
  loop L_ya {
    g = (i + e)[0]:ubyte;
    br_table[B_za, B_za, B_va, B_va, B_za, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_za, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_va, B_ua, ..B_wa](
      g + -9);
    label B_za:
    f[0] = (e = e + 1);
    if (h != e) continue L_ya;
  }
  label B_xa:
  c[8]:int = 3;
  m = f_df(b, c + 32);
  goto B_ta;
  label B_wa:
  if (g == 125) goto B_sa;
  label B_va:
  c[8]:int = 19;
  m = f_df(b, c + 32);
  goto B_ta;
  label B_ua:
  c[8]:int = 18;
  m = f_df(b, c + 32);
  label B_ta:
  if (eqz(d)) goto B_oa;
  if (j) goto B_ra;
  n = m;
  goto B_f;
  label B_sa:
  (b + 8)[0]:int = e + 1;
  if (d) goto B_qa;
  m = 0;
  goto B_oa;
  label B_ra:
  if (l) goto B_ab;
  l = j + 1;
  goto B_pa;
  label B_ab:
  e = k + 8;
  f = k + (l = j + 1);
  o = (k[0]:long ^ -1L) & -9187201950435737472L;
  i = k;
  loop L_bb {
    if (o == 0L) goto B_db;
    p = o;
    goto B_cb;
    label B_db:
    loop L_eb {
      if (e >= f) goto B_pa;
      i = i + -160;
      p = e[0]:long;
      g = e + 8;
      e = g;
      p = p & -9187201950435737472L;
      if (p == -9187201950435737472L) continue L_eb;
    }
    p = p ^ -9187201950435737472L;
    e = g;
    label B_cb:
    o = p + -1L & p;
    g = i + (0 - (i32_wrap_i64(ctz(p)) >> 3)) * 20;
    h = (g + -8)[0]:int;
    if (eqz(h)) continue L_bb;
    f_mi((g + -12)[0]:int, h, 1);
    continue L_bb;
  }
  label B_qa:
  (a + 8)[0]:long = c[2]:long;
  (a + 16)[0]:long = (c + 16 + 8)[0]:long;
  (a + 36)[0]:int = l;
  (a + 32)[0]:int = m;
  (a + 28)[0]:int = k;
  (a + 24)[0]:int = j;
  e = 0;
  goto B_a;
  label B_pa:
  i = j + (e = i32_wrap_i64(i64_extend_i32_u(l) * 20L) + 7 & -8) + 9;
  if (i) goto B_fb;
  n = m;
  goto B_oa;
  label B_fb:
  f_mi(k - e, i, 8);
  n = m;
  label B_oa:
  if (d | eqz(m)) goto B_f;
  br_table[B_ib, B_hb, ..B_gb](m[0]:int)
  label B_ib:
  e = m[2]:int;
  if (eqz(e)) goto B_gb;
  f_mi(m[1]:int, e, 1);
  goto B_gb;
  label B_hb:
  if (m[4]:ubyte != 3) goto B_gb;
  e = m[2]:int;
  call_indirect(e[0]:int, (e[1]:int)[0]:int);
  i = e[1]:int;
  g = i[1]:int;
  if (eqz(g)) goto B_jb;
  f_mi(e[0]:int, g, i[2]:int);
  label B_jb:
  f_mi(m[2]:int, 12, 4);
  label B_gb:
  f_mi(m, 20, 4);
  label B_f:
  a.b = f_gf(n, b);
  label B_b:
  e = 1;
  label B_a:
  a.a = e;
  g_a = c + 128;
}

function f_i(a:{ a:ubyte, b:ubyte, c:int }, b:int):int {
  var d:int_ptr;
  var h:long;
  var f:int;
  var i:long;
  var g:int;
  var e:int;
  var j:long;
  var c:int = g_a - 80;
  g_a = c;
  br_table[B_i, B_h, B_g, B_f, B_e, B_d, ..B_i](a.a)
  label B_i:
  f_jc(c + 72, b[0]:int, 1054372, 4);
  a = 0;
  if (c[72]:ubyte == 4) goto B_a;
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_h:
  d = b[0]:int;
  if (a.b) goto B_k;
  f_jc(c + 72, d, 1054380, 5);
  goto B_j;
  label B_k:
  f_jc(c + 72, d, 1054376, 4);
  label B_j:
  a = 0;
  if (c[72]:ubyte == 4) goto B_a;
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_g:
  e = b[0]:int;
  br_table[B_n, B_m, B_l, ..B_n]((a + 8)[0]:int)
  label B_n:
  d = 20;
  h = (a + 16)[0]:long;
  if (h >= 10000L) goto B_o;
  i = h;
  goto B_b;
  label B_o:
  d = 20;
  loop L_p {
    a = c + 24 + d;
    (a + -4)[0]:short@1 =
      (((b = ((f = i32_wrap_i64(h - (i = h / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((f - b * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    d = d + -4;
    a = h > 99999999L;
    h = i;
    if (a) continue L_p;
    goto B_b;
  }
  label B_m:
  d = 20;
  j = (a + 16)[0]:long;
  h = j + (h = j >> 63L) ^ h;
  if (h >= 10000L) goto B_q;
  i = h;
  goto B_c;
  label B_q:
  d = 20;
  loop L_r {
    a = c + 24 + d;
    (a + -4)[0]:short@1 =
      (((b = ((f = i32_wrap_i64(h - (i = h / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((f - b * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    d = d + -4;
    a = h > 99999999L;
    h = i;
    if (a) continue L_r;
    goto B_c;
  }
  label B_l:
  var k:double = (a + 16)[0]:double;
  if ((f_qf(k) & 255) < 2) goto B_s;
  f_jc(c + 72, e, c + 24, f_w(k, c + 24));
  if (c[72]:ubyte != 4) goto B_t;
  a = 0;
  goto B_a;
  label B_t:
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_s:
  f_jc(c + 72, e, 1054372, 4);
  if (c[72]:ubyte != 4) goto B_u;
  a = 0;
  goto B_a;
  label B_u:
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_f:
  f_ta(c + 72, b, a, (a + 4)[0]:int, (a + 12)[0]:int);
  a = 0;
  if (c[72]:ubyte == 4) goto B_a;
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_e:
  a = f_qa(b, a + 4);
  goto B_a;
  label B_d:
  d = (a + 12)[0]:int;
  (b + 16)[0]:byte = 0;
  f = 1;
  b[1]:int = (e = b[1]:int) + 1;
  f_jc(c + 72, g = b[0]:int, 1054419, 1);
  if (c[72]:ubyte != 4) goto B_y;
  if (d) goto B_z;
  b[1]:int = e;
  f_jc(c + 72, g, 1054420, 1);
  if (c[72]:ubyte != 4) goto B_x;
  f = 0;
  label B_z:
  c[20]:byte = f;
  c[4]:int = b;
  e = a.c;
  (c + 48)[0]:int = (f = (a + 8)[0]:int);
  (c + 44)[0]:int = e;
  (c + 24 + 8)[0]:int = f;
  c[7]:int = e;
  c[10]:int = (a = eqz(f) << 1);
  c[6]:int = a;
  if (eqz(f)) goto B_v;
  if (eqz(d)) goto B_v;
  e = c + 24 | 4;
  loop L_aa {
    c[14]:int = d + -1;
    br_table[B_ca, B_ba, B_w, ..B_ba](a)
    label B_ca:
    d = c[8]:int;
    a = c[7]:int;
    if (eqz(a)) goto B_da;
    g = a + -1;
    f = a & 7;
    if (eqz(f)) goto B_ea;
    loop L_fa {
      a = a + -1;
      d = d[102];
      f = f + -1;
      if (f) continue L_fa;
    }
    label B_ea:
    if (g < 7) goto B_da;
    loop L_ga {
      d = 
        (((((((d[102])[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
      a = a + -8;
      if (a) continue L_ga;
    }
    label B_da:
    c[9]:int = 0;
    c[8]:int = d;
    c[3]:long = 1L;
    label B_ba:
    f_sc(c + 8, e);
    a = c[2]:int;
    if (eqz(a)) goto B_v;
    d = c[3]:int;
    a = f_vc(c + 16, a);
    if (a) goto B_a;
    f_jc(c + 64, (b = c[4]:int)[0]:int, 1054427, 2);
    if (c[64]:ubyte == 4) goto B_ha;
    c[9]:long = c[8]:long;
    a = f_cg(c + 72);
    goto B_a;
    label B_ha:
    a = f_i(d, b);
    if (a) goto B_a;
    b[16]:byte = 1;
    d = c[14]:int;
    if (eqz(d)) goto B_v;
    a = c[6]:int;
    continue L_aa;
  }
  label B_y:
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_x:
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_w:
  f_rf(1055948, 43, 1056088);
  unreachable;
  label B_v:
  if (c[20]:ubyte) goto B_ia;
  a = 0;
  goto B_a;
  label B_ia:
  b[1]:int = (a = b[1]:int + -1);
  d = b[0]:int;
  if (eqz(b[16]:ubyte)) goto B_la;
  f_jc(c + 24, d, 1054424, 1);
  if (c[24]:ubyte != 4) goto B_ma;
  if (eqz(a)) goto B_la;
  f = b[3]:int;
  b = b[2]:int;
  loop L_oa {
    f_jc(c + 24, d, b, f);
    if (c[24]:ubyte != 4) goto B_na;
    a = a + -1;
    if (eqz(a)) goto B_la;
    continue L_oa;
  }
  label B_na:
  h = c[3]:long;
  if ((i32_wrap_i64(h) & 255) == 4) goto B_la;
  goto B_ja;
  label B_ma:
  c[9]:long = (h = c[3]:long);
  d = i32_wrap_i64(h);
  goto B_ka;
  label B_la:
  f_jc(c + 72, d, 1054420, 1);
  d = c[72]:ubyte;
  label B_ka:
  a = 0;
  if ((d & 255) == 4) goto B_a;
  h = c[9]:long;
  label B_ja:
  c[3]:long = h;
  a = f_cg(c + 24);
  goto B_a;
  label B_c:
  a = i32_wrap_i64(i);
  if (a > 99) goto B_qa;
  f = a;
  goto B_pa;
  label B_qa:
  (c + 24 + (d = d + -2))[0]:short@1 =
    ((a - (f = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_pa:
  if (f < 10) goto B_sa;
  (c + 24 + (a = d + -2))[0]:short@1 = ((f << 1) + 1054000)[0]:ushort@1;
  goto B_ra;
  label B_sa:
  (c + 24 + (a = d + -1))[0]:byte = f + 48;
  label B_ra:
  if (j > -1L) goto B_ta;
  (c + 24 + (a = a + -1))[0]:byte = 45;
  label B_ta:
  f_jc(c + 72, e, c + 24 + a, 20 - a);
  a = 0;
  if (c[72]:ubyte == 4) goto B_a;
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  goto B_a;
  label B_b:
  a = i32_wrap_i64(i);
  if (a > 99) goto B_va;
  f = a;
  goto B_ua;
  label B_va:
  (c + 24 + (d = d + -2))[0]:short@1 =
    ((a - (f = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_ua:
  if (f < 10) goto B_xa;
  (c + 24 + (a = d + -2))[0]:short@1 = ((f << 1) + 1054000)[0]:ushort@1;
  goto B_wa;
  label B_xa:
  (c + 24 + (a = d + -1))[0]:byte = f + 48;
  label B_wa:
  f_jc(c + 72, e, c + 24 + a, 20 - a);
  a = 0;
  if (c[72]:ubyte == 4) goto B_a;
  c[3]:long = c[9]:long;
  a = f_cg(c + 24);
  label B_a:
  g_a = c + 80;
  return a;
}

function f_j(a:int, b:int_ptr):int {
  var d:int_ptr;
  var j:long;
  var f:int;
  var k:long;
  var g:int;
  var e:int;
  var l:long;
  var c:int = g_a - 64;
  g_a = c;
  br_table[B_i, B_h, B_g, B_f, B_e, B_d, ..B_i](a[0]:ubyte)
  label B_i:
  f_jc(c + 56, b[0], 1054372, 4);
  a = 0;
  if (c[56]:ubyte == 4) goto B_a;
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_h:
  d = b[0];
  if (a[1]:ubyte) goto B_k;
  f_jc(c + 56, d, 1054380, 5);
  goto B_j;
  label B_k:
  f_jc(c + 56, d, 1054376, 4);
  label B_j:
  a = 0;
  if (c[56]:ubyte == 4) goto B_a;
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_g:
  br_table[B_n, B_m, B_l, ..B_n]((a + 8)[0]:int)
  label B_n:
  e = b[0];
  d = 20;
  j = (a + 16)[0]:long;
  if (j >= 10000L) goto B_o;
  k = j;
  goto B_b;
  label B_o:
  d = 20;
  loop L_p {
    a = c + 8 + d;
    (a + -4)[0]:short@1 =
      (((b = ((f = i32_wrap_i64(j - (k = j / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((f - b * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    d = d + -4;
    a = j > 99999999L;
    j = k;
    if (a) continue L_p;
    goto B_b;
  }
  label B_m:
  e = b[0];
  d = 20;
  l = (a + 16)[0]:long;
  j = l + (j = l >> 63L) ^ j;
  if (j >= 10000L) goto B_q;
  k = j;
  goto B_c;
  label B_q:
  d = 20;
  loop L_r {
    a = c + 8 + d;
    (a + -4)[0]:short@1 =
      (((b = ((f = i32_wrap_i64(j - (k = j / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1054000)[0]:ushort@1;
    (a + -2)[0]:short@1 =
      (((f - b * 100 & 65535) << 1) + 1054000)[0]:ushort@1;
    d = d + -4;
    a = j > 99999999L;
    j = k;
    if (a) continue L_r;
    goto B_c;
  }
  label B_l:
  var m:double = (a + 16)[0]:double;
  if ((f_qf(m) & 255) < 2) goto B_s;
  f_jc(c + 56, b[0], c + 8, f_w(m, c + 8));
  if (c[56]:ubyte != 4) goto B_t;
  a = 0;
  goto B_a;
  label B_t:
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_s:
  f_jc(c + 56, b[0], 1054372, 4);
  if (c[56]:ubyte != 4) goto B_u;
  a = 0;
  goto B_a;
  label B_u:
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_f:
  f_sa(c + 56, b, a, (a + 4)[0]:int, (a + 12)[0]:int);
  a = 0;
  if (c[56]:ubyte == 4) goto B_a;
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_e:
  a = f_kc(b, a + 4);
  goto B_a;
  label B_d:
  d = (a + 12)[0]:int;
  e = 1;
  f_jc(c + 56, g = b[0], 1054419, 1);
  if (c[56]:ubyte != 4) goto B_y;
  if (d) goto B_z;
  f_jc(c + 56, g, 1054420, 1);
  if (c[56]:ubyte != 4) goto B_x;
  e = 0;
  label B_z:
  f = a[1]:int;
  (c + 32)[0]:int = (a = (a + 8)[0]:int);
  (c + 28)[0]:int = f;
  (c + 8 + 8)[0]:int = a;
  c[3]:int = f;
  c[6]:int = (f = eqz(a) << 1);
  c[2]:int = f;
  if (eqz(a)) goto B_v;
  if (eqz(d)) goto B_v;
  var h:int = c + 8 | 4;
  loop L_aa {
    c[10]:int = d + -1;
    br_table[B_ca, B_ba, B_w, ..B_ba](c[2]:int)
    label B_ca:
    d = c[4]:int;
    a = c[3]:int;
    if (eqz(a)) goto B_da;
    var i:int = a + -1;
    f = a & 7;
    if (eqz(f)) goto B_ea;
    loop L_fa {
      a = a + -1;
      d = d[102];
      f = f + -1;
      if (f) continue L_fa;
    }
    label B_ea:
    if (i < 7) goto B_da;
    loop L_ga {
      d = 
        (((((((d[102])[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
      a = a + -8;
      if (a) continue L_ga;
    }
    label B_da:
    c[5]:int = 0;
    c[4]:int = d;
    c[1]:long = 1L;
    label B_ba:
    f_sc(c, h);
    a = c[0]:int;
    if (eqz(a)) goto B_v;
    d = c[1]:int;
    if ((e & 255) == 1) goto B_ha;
    f_jc(c + 48, g, 1054418, 1);
    if (c[48]:ubyte == 4) goto B_ha;
    c[7]:long = c[6]:long;
    a = f_cg(c + 56);
    goto B_a;
    label B_ha:
    f_sa(c + 48, b, a, a[0]:int, a[2]:int);
    if (c[48]:ubyte == 4) goto B_ia;
    c[7]:long = c[6]:long;
    a = f_cg(c + 56);
    goto B_a;
    label B_ia:
    f_jc(c + 48, g, 1054421, 1);
    if (c[48]:ubyte == 4) goto B_ja;
    c[7]:long = c[6]:long;
    a = f_cg(c + 56);
    goto B_a;
    label B_ja:
    a = f_j(d, b);
    if (a) goto B_a;
    e = 2;
    d = c[10]:int;
    if (d) continue L_aa;
    goto B_v;
  }
  label B_y:
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_x:
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_w:
  f_rf(1055948, 43, 1056088);
  unreachable;
  label B_v:
  if (e & 255) goto B_ka;
  a = 0;
  goto B_a;
  label B_ka:
  f_jc(c + 56, g, 1054420, 1);
  a = 0;
  if (c[56]:ubyte == 4) goto B_a;
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_c:
  a = i32_wrap_i64(k);
  if (a > 99) goto B_ma;
  f = a;
  goto B_la;
  label B_ma:
  (c + 8 + (d = d + -2))[0]:short@1 =
    ((a - (f = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_la:
  if (f < 10) goto B_oa;
  (c + 8 + (a = d + -2))[0]:short@1 = ((f << 1) + 1054000)[0]:ushort@1;
  goto B_na;
  label B_oa:
  (c + 8 + (a = d + -1))[0]:byte = f + 48;
  label B_na:
  if (l > -1L) goto B_pa;
  (c + 8 + (a = a + -1))[0]:byte = 45;
  label B_pa:
  f_jc(c + 56, e, c + 8 + a, 20 - a);
  a = 0;
  if (c[56]:ubyte == 4) goto B_a;
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  goto B_a;
  label B_b:
  a = i32_wrap_i64(k);
  if (a > 99) goto B_ra;
  f = a;
  goto B_qa;
  label B_ra:
  (c + 8 + (d = d + -2))[0]:short@1 =
    ((a - (f = a / 100) * 100 << 1) + 1054000)[0]:ushort@1;
  label B_qa:
  if (f < 10) goto B_ta;
  (c + 8 + (a = d + -2))[0]:short@1 = ((f << 1) + 1054000)[0]:ushort@1;
  goto B_sa;
  label B_ta:
  (c + 8 + (a = d + -1))[0]:byte = f + 48;
  label B_sa:
  f_jc(c + 56, e, c + 8 + a, 20 - a);
  a = 0;
  if (c[56]:ubyte == 4) goto B_a;
  c[1]:long = c[7]:long;
  a = f_cg(c + 8);
  label B_a:
  g_a = c + 64;
  return a;
}

function f_k(a:int_ptr, b:int, c:int_ptr):int {
  var h:int;
  var i:int;
  var j:int;
  var f:int_ptr;
  var g:int;
  var d:int = g_a - 32;
  g_a = d;
  var e:ubyte_ptr = a[0];
  f = a + 8;
  g = f[0];
  if (g < (h = (a + 4)[0]:int)) goto B_b;
  if (g > h) goto B_c;
  if (g) goto B_e;
  c = 1;
  h = 0;
  goto B_d;
  label B_e:
  a = g & 3;
  if (g + -1 >= 3) goto B_g;
  h = 0;
  c = 1;
  goto B_f;
  label B_g:
  g = g & -4;
  c = 1;
  h = 0;
  loop L_h {
    h = 
      select_if(0,
                select_if(1,
                          select_if(2,
                                    select_if(3, h + 4, f = e[0] == 10),
                                    b = (e + 1)[0]:ubyte == 10),
                          i = (e + 2)[0]:ubyte == 10),
                j = (e + 3)[0]:ubyte == 10);
    c = c + f + b + i + j;
    e = e + 4;
    g = g + -4;
    if (g) continue L_h;
  }
  label B_f:
  if (eqz(a)) goto B_d;
  loop L_i {
    h = select_if(0, h + 1, g = e[0] == 10);
    e = e + 1;
    c = c + g;
    a = a + -1;
    if (a) continue L_i;
  }
  label B_d:
  d[4]:int = 4;
  e = f_tf(d + 16, c, h);
  goto B_a;
  label B_c:
  f_sj(g, h, 1053316);
  unreachable;
  label B_b:
  h = 1;
  f[0] = (i = g + 1);
  br_table[B_k, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_m, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_l, B_j, B_j, B_j, B_j, B_j, B_n, B_j, B_j, B_j, B_o, B_j, B_j, B_j, B_j, B_j, B_j, B_j, B_p, B_j, B_j, B_j, B_q, B_j, B_r, B_s, ..B_j](
    (e + g)[0]:ubyte + -34)
  label B_s:
  f_ea(d + 8, a);
  if (d[4]:ushort) goto B_v;
  h = d[5]:ushort;
  e = h & 64512;
  if (e == 55296) goto B_x;
  if (e != 56320) goto B_y;
  if (eqz(b)) goto B_t;
  d[4]:int = 17;
  e = f_gc(a[0], (a + 4)[0]:int, (a + 8)[0]:int, d + 16);
  goto B_a;
  label B_y:
  if ((h ^ -1058816) > -1112065) goto B_w;
  f_rf(1053180, 43, 1053668);
  unreachable;
  label B_x:
  f_bc(d + 16, g = a[0], f = (a + 4)[0]:int, e = (a + 8)[0]:int);
  if (d[16]:ubyte) goto B_u;
  if (d[17]:ubyte != 92) goto B_da;
  (a + 8)[0]:int = (i = e + 1);
  f_bc(d + 16, g, f, i);
  if (d[16]:ubyte) goto B_ca;
  if (d[17]:ubyte != 117) goto B_ba;
  (a + 8)[0]:int = e + 2;
  f_ea(d + 16, a);
  if (d[8]:ushort) goto B_aa;
  e = d[9]:ushort;
  if ((e + 8192 & 65535) < 64512) goto B_z;
  h = select_if(
        1114112,
        e = ((h + 10240 & 65535) << 10 | (e + 9216 & 65535)) + 65536,
        (e ^ 55296) + -1114112 < -1112064);
  if (h != 1114112) goto B_w;
  d[4]:int = 14;
  e = f_gc(a[0], (a + 4)[0]:int, (a + 8)[0]:int, d + 16);
  goto B_a;
  label B_da:
  if (eqz(b)) goto B_t;
  (a + 8)[0]:int = (e = e + 1);
  d[4]:int = 20;
  e = f_gc(g, f, e, d + 16);
  goto B_a;
  label B_ca:
  e = d[5]:int;
  goto B_a;
  label B_ba:
  if (b) goto B_ea;
  f_de(c, h);
  e = f_k(a, 0, c);
  goto B_a;
  label B_ea:
  (a + 8)[0]:int = (e = e + 2);
  d[4]:int = 20;
  e = f_gc(g, f, e, d + 16);
  goto B_a;
  label B_aa:
  e = d[5]:int;
  goto B_a;
  label B_z:
  d[4]:int = 17;
  e = f_gc(a[0], (a + 4)[0]:int, (a + 8)[0]:int, d + 16);
  goto B_a;
  label B_w:
  e = 0;
  d[4]:int = 0;
  f_fd(d, h, d + 16);
  f_of(c, d[0]:int, d[1]:int);
  goto B_a;
  label B_v:
  e = d[3]:int;
  goto B_a;
  label B_u:
  e = d[5]:int;
  goto B_a;
  label B_t:
  f_de(c, h);
  e = 0;
  goto B_a;
  label B_r:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_fa;
  f_id(c, e);
  e = c[2];
  label B_fa:
  (c[0] + e)[0]:byte = 9;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_q:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_ga;
  f_id(c, e);
  e = c[2];
  label B_ga:
  (c[0] + e)[0]:byte = 13;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_p:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_ha;
  f_id(c, e);
  e = c[2];
  label B_ha:
  (c[0] + e)[0]:byte = 10;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_o:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_ia;
  f_id(c, e);
  e = c[2];
  label B_ia:
  (c[0] + e)[0]:byte = 12;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_n:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_ja;
  f_id(c, e);
  e = c[2];
  label B_ja:
  (c[0] + e)[0]:byte = 8;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_m:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_ka;
  f_id(c, e);
  e = c[2];
  label B_ka:
  (c[0] + e)[0]:byte = 47;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_l:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_la;
  f_id(c, e);
  e = c[2];
  label B_la:
  (c[0] + e)[0]:byte = 92;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_k:
  e = c[2];
  if (e != (c + 4)[0]:int) goto B_ma;
  f_id(c, e);
  e = c[2];
  label B_ma:
  (c[0] + e)[0]:byte = 34;
  c[2] = c[2] + 1;
  e = 0;
  goto B_a;
  label B_j:
  a = i & 3;
  if (g >= 3) goto B_oa;
  c = 0;
  goto B_na;
  label B_oa:
  g = i & -4;
  h = 1;
  c = 0;
  loop L_pa {
    c = 
      select_if(0,
                select_if(1,
                          select_if(2,
                                    select_if(3, c + 4, f = e[0] == 10),
                                    b = (e + 1)[0]:ubyte == 10),
                          i = (e + 2)[0]:ubyte == 10),
                j = (e + 3)[0]:ubyte == 10);
    h = h + f + b + i + j;
    e = e + 4;
    g = g + -4;
    if (g) continue L_pa;
  }
  label B_na:
  if (eqz(a)) goto B_qa;
  loop L_ra {
    c = select_if(0, c + 1, g = e[0] == 10);
    e = e + 1;
    h = h + g;
    a = a + -1;
    if (a) continue L_ra;
  }
  label B_qa:
  d[4]:int = 11;
  e = f_tf(d + 16, h, c);
  label B_a:
  g_a = d + 32;
  return e;
}

function f_l(a:{ a:long, b:int }, b:long, c:int) {
  var h:int;
  var i:int;
  var l:long;
  var m:long;
  var n:long;
  var g:int;
  var o:long;
  var p:long;
  var d:long_ptr = g_a - 288;
  g_a = d;
  var e:int = b != 0L | c < 2;
  var j:long = select_if(b | 4503599627370496L, b, c);
  b = j << 2L;
  var k:long = j & 1L;
  var f:int = 0;
  c = select_if(c + -1077, -1076, c);
  if (c > -1) goto B_g;
  g = 0 - c;
  f = 1;
  f_ce(
    d + 272,
    j = 
      ((i = (g = g - (h = (c * -732923 >> 20) - (g > 1))) << 4) + 1061608)[0]:long,
    0L,
    l = b | 2L,
    0L);
  f_ce(d + 256, m = (i + 1061616)[0]:long, 0L, l, 0L);
  f_ze(d + 240,
       n = (l = (d + 272 + 8)[0]:long) + d[32],
       (d + 256 + 8)[0]:long + i64_extend_i32_u(n < l),
       g = h - (g * 1217359 >> 19) + 60 & 127);
  f_ce(d + 176, j, 0L, l = b + (i64_extend_i32_u(e) ^ -1L), 0L);
  f_ce(d + 160, m, 0L, l, 0L);
  f_ze(d + 144,
       n = (l = (d + 176 + 8)[0]:long) + d[20],
       (d + 160 + 8)[0]:long + i64_extend_i32_u(n < l),
       g);
  f_ce(d + 224, j, 0L, b, 0L);
  f_ce(d + 208, m, 0L, b, 0L);
  f_ze(d + 192,
       m = (j = (d + 224 + 8)[0]:long) + d[26],
       (d + 208 + 8)[0]:long + i64_extend_i32_u(m < j),
       g);
  i = h + c;
  m = d[24];
  l = d[18];
  o = d[30];
  if (h < 2) goto B_d;
  g = 0;
  if (h >= 63) goto B_f;
  f = eqz(b & (-1L << i64_extend_i32_u(h) ^ -1L));
  goto B_e;
  label B_g:
  f_ce(
    d + 128,
    m = ((g = (i = (c * 78913 >> 18) - (c > 3)) << 4) + 1056136)[0]:long,
    0L,
    j = b | 2L,
    0L);
  f_ce(d + 112, l = (g + 1056144)[0]:long, 0L, j, 0L);
  f_ze(d + 96,
       o = (n = (d + 128 + 8)[0]:long) + d[14],
       (d + 112 + 8)[0]:long + i64_extend_i32_u(o < n),
       c = i - c + (i * 1217359 >> 19) + 61 & 127);
  f_ce(d + 32, m, 0L, n = b + ((p = i64_extend_i32_u(e)) ^ -1L), 0L);
  f_ce(d + 16, l, 0L, n, 0L);
  f_ze(d,
       o = (n = (d + 32 + 8)[0]:long) + d[2],
       (d + 16 + 8)[0]:long + i64_extend_i32_u(o < n),
       c);
  f_ce(d + 80, m, 0L, b, 0L);
  f_ce(d + 64, l, 0L, b, 0L);
  f_ze(d + 48,
       l = (m = (d + 80 + 8)[0]:long) + d[8],
       (d + 64 + 8)[0]:long + i64_extend_i32_u(l < m),
       c);
  m = d[6];
  l = d[0];
  o = d[12];
  if (i <= 21) goto B_h;
  g = 0;
  goto B_e;
  label B_h:
  if (i32_wrap_i64(b / 5L) * -5 != 0 - i32_wrap_i64(b)) goto B_i;
  c = -1;
  loop L_j {
    f = i32_wrap_i64(b);
    c = c + 1;
    b = b / 5L;
    if (i32_wrap_i64(b) * -5 == 0 - f) continue L_j;
  }
  f = c >= i;
  g = 0;
  goto B_e;
  label B_i:
  if (eqz(k)) goto B_k;
  c = -1;
  loop L_l {
    f = i32_wrap_i64(j);
    c = c + 1;
    j = j / 5L;
    if (i32_wrap_i64(j) * -5 == 0 - f) continue L_l;
  }
  o = o - i64_extend_i32_u(c >= i);
  f = 0;
  g = 0;
  goto B_e;
  label B_k:
  b = (p ^ -1L) + b;
  c = -1;
  loop L_m {
    f = i32_wrap_i64(b);
    c = c + 1;
    b = b / 5L;
    if (i32_wrap_i64(b) * -5 == 0 - f) continue L_m;
  }
  g = c >= i;
  label B_f:
  f = 0;
  label B_e:
  if (f) goto B_b;
  if (eqz(g)) goto B_c;
  goto B_b;
  label B_d:
  o = o - k;
  g = eqz(k) & e;
  goto B_b;
  label B_c:
  c = 0;
  j = o / 100L;
  if (j > (n = l / 100L)) goto B_o;
  n = l;
  j = o;
  b = m;
  f = 0;
  goto B_n;
  label B_o:
  b = m / 100L;
  f = i32_wrap_i64(b) * -100 + i32_wrap_i64(m) > 49;
  c = 2;
  label B_n:
  j = j / 10L;
  if (j <= (m = n / 10L)) goto B_p;
  loop L_q {
    c = c + 1;
    l = b;
    b = l / 10L;
    j = j / 10L;
    if (j > (m = (n = m) / 10L)) continue L_q;
  }
  f = i32_wrap_i64(b) * -10 + i32_wrap_i64(l) > 4;
  label B_p:
  b = b + i64_extend_i32_u(b == n | f);
  goto B_a;
  label B_b:
  e = 0;
  n = o / 10L;
  if (n > (o = l / 10L)) goto B_s;
  c = 0;
  b = l;
  j = m;
  goto B_r;
  label B_s:
  c = 0;
  e = 0;
  loop L_t {
    g = g & i32_wrap_i64(b = o) * -10 == 0 - i32_wrap_i64(l);
    c = c + 1;
    f = eqz(e & 255) & f;
    j = m / 10L;
    e = i32_wrap_i64(j) * -10 + i32_wrap_i64(m);
    m = j;
    l = b;
    n = n / 10L;
    if (n > (o = b / 10L)) continue L_t;
  }
  label B_r:
  if (eqz(g)) goto B_w;
  m = b / 10L;
  if (i32_wrap_i64(m) * -10 == 0 - i32_wrap_i64(b)) goto B_v;
  label B_w:
  l = j;
  goto B_u;
  label B_v:
  loop L_x {
    h = i32_wrap_i64(m);
    c = c + 1;
    f = eqz(e & 255) & f;
    l = j / 10L;
    e = i32_wrap_i64(l) * -10 + i32_wrap_i64(j);
    b = m;
    n = m / 10L;
    m = n;
    j = l;
    if (i32_wrap_i64(n) * -10 == 0 - h) continue L_x;
  }
  label B_u:
  j = 1L;
  if (l != b) goto B_z;
  if ((eqz(k) & g) != 1) goto B_y;
  label B_z:
  j = 
    i64_extend_i32_u(
      (select_if(select_if(select_if(4, 5, eqz(l & 1L)), e, (e & 255) == 5),
                 e,
                 f) &
       255) >
      4);
  label B_y:
  b = j + l;
  label B_a:
  a.a = b;
  a.b = c + i;
  g_a = d + 288;
}

function f_m(a:int, b:int, c:int, d:int, e:int) {
  var i:int;
  var m:int;
  var h:int;
  var l:int;
  var p:long;
  var j:int;
  var k:int;
  var n:int;
  var g:ubyte_ptr;
  if (eqz(e)) goto B_c;
  var f:int = 1;
  g = 0;
  if (e != 1) goto B_e;
  h = 1;
  i = 0;
  goto B_d;
  label B_e:
  j = 1;
  k = 0;
  l = 1;
  g = 0;
  f = 1;
  loop L_f {
    m = l;
    l = g + k;
    if (l >= e) goto B_i;
    j = (d + j)[0]:ubyte & 255;
    if (j < (l = (d + l)[0]:ubyte)) goto B_j;
    if (j == l) goto B_h;
    f = 1;
    l = m + 1;
    g = 0;
    k = m;
    goto B_g;
    label B_j:
    l = m + g + 1;
    f = l - k;
    g = 0;
    goto B_g;
    label B_i:
    f_ne(l, e, 1073568);
    unreachable;
    label B_h:
    g = select_if(0, l = g + 1, j = l == f);
    l = select_if(l, 0, j) + m;
    label B_g:
    j = l + g;
    if (j < e) continue L_f;
  }
  j = 1;
  i = 0;
  l = 1;
  g = 0;
  h = 1;
  loop L_k {
    m = l;
    l = g + i;
    if (l >= e) goto B_n;
    j = (d + j)[0]:ubyte & 255;
    if (j > (l = (d + l)[0]:ubyte)) goto B_o;
    if (j == l) goto B_m;
    h = 1;
    l = m + 1;
    g = 0;
    i = m;
    goto B_l;
    label B_o:
    l = m + g + 1;
    h = l - i;
    g = 0;
    goto B_l;
    label B_n:
    f_ne(l, e, 1073568);
    unreachable;
    label B_m:
    g = select_if(0, l = g + 1, j = l == h);
    l = select_if(l, 0, j) + m;
    label B_l:
    j = l + g;
    if (j < e) continue L_k;
  }
  g = k;
  label B_d:
  n = select_if(g, i, l = g > i);
  if (n > e) goto B_p;
  l = select_if(f, h, l);
  g = l + n;
  if (g < l) goto B_q;
  if (g > e) goto B_r;
  if (eqz(f_ck(d, d + l, n))) goto B_s;
  i = n > (j = e - n);
  l = e & 3;
  if (e + -1 >= 3) goto B_t;
  p = 0L;
  g = d;
  goto B_b;
  label B_t:
  m = e & -4;
  p = 0L;
  g = d;
  loop L_u {
    p = 
      1L << (g + 3)[0]:ubyte |
      (1L << (g + 2)[0]:ubyte | (1L << (g + 1)[0]:ubyte | (1L << g[0] | p)));
    g = g + 4;
    m = m + -4;
    if (m) continue L_u;
    goto B_b;
  }
  label B_s:
  i = 1;
  g = 0;
  j = 1;
  f = 0;
  loop L_w {
    m = j;
    h = m + g;
    if (h >= e) goto B_v;
    j = e - g + (m ^ -1);
    if (j >= e) goto B_aa;
    k = (g ^ -1) + e - f;
    if (k >= e) goto B_z;
    j = (d + j)[0]:ubyte & 255;
    if (j < (k = (d + k)[0]:ubyte)) goto B_ba;
    if (j == k) goto B_y;
    j = m + 1;
    g = 0;
    i = 1;
    f = m;
    goto B_x;
    label B_ba:
    j = h + 1;
    i = j - f;
    g = 0;
    goto B_x;
    label B_aa:
    f_ne(j, e, 1073584);
    unreachable;
    label B_z:
    f_ne(k, e, 1073600);
    unreachable;
    label B_y:
    g = select_if(0, j = g + 1, k = j == i);
    j = select_if(j, 0, k) + m;
    label B_x:
    if (i != l) continue L_w;
  }
  label B_v:
  i = 1;
  g = 0;
  j = 1;
  h = 0;
  loop L_da {
    m = j;
    var o:int = m + g;
    if (o >= e) goto B_ca;
    j = e - g + (m ^ -1);
    if (j >= e) goto B_ha;
    k = (g ^ -1) + e - h;
    if (k >= e) goto B_ga;
    j = (d + j)[0]:ubyte & 255;
    if (j > (k = (d + k)[0]:ubyte)) goto B_ia;
    if (j == k) goto B_fa;
    j = m + 1;
    g = 0;
    i = 1;
    h = m;
    goto B_ea;
    label B_ia:
    j = o + 1;
    i = j - h;
    g = 0;
    goto B_ea;
    label B_ha:
    f_ne(j, e, 1073584);
    unreachable;
    label B_ga:
    f_ne(k, e, 1073600);
    unreachable;
    label B_fa:
    g = select_if(0, j = g + 1, k = j == i);
    j = select_if(j, 0, k) + m;
    label B_ea:
    if (i != l) continue L_da;
  }
  label B_ca:
  if (l > e) goto B_ja;
  k = e - select_if(f, h, f > h);
  i = 0;
  if (l) goto B_la;
  p = 0L;
  l = 0;
  goto B_ka;
  label B_la:
  m = l & 3;
  if (l + -1 >= 3) goto B_na;
  p = 0L;
  g = d;
  goto B_ma;
  label B_na:
  j = l & -4;
  p = 0L;
  g = d;
  loop L_oa {
    p = 
      1L << (g + 3)[0]:ubyte |
      (1L << (g + 2)[0]:ubyte | (1L << (g + 1)[0]:ubyte | (1L << g[0] | p)));
    g = g + 4;
    j = j + -4;
    if (j) continue L_oa;
  }
  label B_ma:
  if (eqz(m)) goto B_ka;
  loop L_pa {
    p = 1L << g[0] | p;
    g = g + 1;
    m = m + -1;
    if (m) continue L_pa;
  }
  label B_ka:
  g = e;
  goto B_a;
  label B_ja:
  f_sj(l, e, e);
  unreachable;
  label B_r:
  f_sj(g, e, e);
  unreachable;
  label B_q:
  f_tj(l, g, e);
  unreachable;
  label B_p:
  f_sj(n, e, e);
  unreachable;
  label B_c:
  a[14]:int = d;
  a[12]:int = b;
  a[14]:byte = 0;
  a[0]:long = 0L;
  (a + 60)[0]:int = 0;
  (a + 52)[0]:int = c;
  (a + 12)[0]:short = 257;
  (a + 8)[0]:int = c;
  return ;
  label B_b:
  m = select_if(n, j, i);
  if (eqz(l)) goto B_qa;
  loop L_ra {
    p = 1L << g[0] | p;
    g = g + 1;
    l = l + -1;
    if (l) continue L_ra;
  }
  label B_qa:
  l = m + 1;
  i = -1;
  k = n;
  g = -1;
  label B_a:
  a[14]:int = d;
  a[12]:int = b;
  a[0]:int = 1;
  (a + 60)[0]:int = e;
  (a + 52)[0]:int = c;
  (a + 40)[0]:int = g;
  (a + 36)[0]:int = i;
  (a + 32)[0]:int = c;
  (a + 28)[0]:int = 0;
  (a + 24)[0]:int = l;
  (a + 20)[0]:int = k;
  (a + 16)[0]:int = n;
  (a + 8)[0]:long@4 = p;
}

function f_n(a:{ a:int, b:int }):int {
  var c:int;
  var d:int;
  var n:int;
  var m:int;
  var k:int;
  var g:{ a:int, b:int, c:int, d:int, e:int }
  var h:int;
  var j:int;
  var e:int;
  var f:int;
  var l:int;
  var i:int;
  var b:int = g_a - 80;
  g_a = b;
  f_m(b + 16, c = a.a, d = (a + 8)[0]:int, 1055588, 9);
  if (b[4]:int) goto B_g;
  if ((b + 30)[0]:ubyte) goto B_d;
  e = b[29]:ubyte;
  f = e ^ 1;
  g = b[6]:int;
  if (eqz(g)) goto B_f;
  h = (b + 68)[0]:int;
  i = b[16]:int;
  j = eqz(e);
  loop L_j {
    if (g < h) goto B_l;
    if (h == g) goto B_k;
    goto B_a;
    label B_l:
    if ((i + g)[0]:byte < -64) goto B_a;
    label B_k:
    k = i + g;
    e = (k + -1)[0]:ubyte;
    l = (e << 24) >> 24;
    if (l > -1) goto B_m;
    e = (k + -2)[0]:ubyte;
    m = (e << 24) >> 24;
    if (m <= -65) goto B_o;
    e = e & 31;
    goto B_n;
    label B_o:
    e = (k + -3)[0]:ubyte;
    n = (e << 24) >> 24;
    if (n <= -65) goto B_q;
    e = e & 15;
    goto B_p;
    label B_q:
    e = ((k + -4)[0]:ubyte & 7) << 6 | (n & 63);
    label B_p:
    e = e << 6 | (m & 63);
    label B_n:
    e = e << 6 | (l & 63);
    label B_m:
    if (j & 1) goto B_r;
    j = f;
    goto B_i;
    label B_r:
    if (e == 1114112) goto B_h;
    k = -1;
    if (e < 128) goto B_s;
    k = -2;
    if (e < 2048) goto B_s;
    k = select_if(-3, -4, e < 65536);
    label B_s:
    j = 0;
    f = 0;
    g = k + g;
    if (g) continue L_j;
  }
  g = 0;
  label B_i:
  b[29]:byte = j;
  goto B_c;
  label B_h:
  b[29]:byte = f;
  b[6]:int = g;
  goto B_e;
  label B_g:
  g = b + 16 + 8;
  e = (b + 76)[0]:int;
  k = (b + 68)[0]:int;
  j = b[18]:int;
  h = b[16]:int;
  if ((b + 52)[0]:int == -1) goto B_t;
  f_ab(b, g, h, k, j, e, 0);
  goto B_b;
  label B_t:
  f_ab(b, g, h, k, j, e, 1);
  goto B_b;
  label B_f:
  b[29]:byte = f;
  g = 0;
  if (e) goto B_c;
  label B_e:
  b[30]:byte = 1;
  label B_d:
  b[0]:int = 0;
  goto B_b;
  label B_c:
  (b + 8)[0]:int = g;
  b[1]:int = g;
  b[0]:int = 1;
  label B_b:
  if (eqz(b[0]:int)) goto B_ca;
  j = b[1]:int;
  l = j + 9;
  g = l;
  loop L_da {
    if (eqz(g)) goto B_ea;
    if (d > g) goto B_fa;
    if (d == g) goto B_ea;
    goto B_u;
    label B_fa:
    if ((c + g)[0]:byte <= -65) goto B_u;
    label B_ea:
    k = d == g;
    if (eqz(k)) goto B_ia;
    h = d;
    goto B_ha;
    label B_ia:
    if (((c + g)[0]:ubyte + -48 & 255) < 10) goto B_ga;
    h = g;
    label B_ha:
    if (eqz(g)) goto B_ja;
    if (d > h) goto B_ka;
    if (k) goto B_ja;
    goto B_v;
    label B_ka:
    if ((c + h)[0]:byte <= -65) goto B_v;
    label B_ja:
    e = 1;
    if (d - h < 8) goto B_ba;
    f = c + h;
    if (f[0]:long@1 != 2336925607749706528L) goto B_ba;
    m = h + 8;
    e = m;
    loop L_la {
      if (eqz(e)) goto B_ma;
      if (d > e) goto B_na;
      if (d == e) goto B_ma;
      goto B_w;
      label B_na:
      if ((c + e)[0]:byte <= -65) goto B_w;
      label B_ma:
      i = d == e;
      if (eqz(i)) goto B_qa;
      n = d;
      goto B_pa;
      label B_qa:
      if (((c + e)[0]:ubyte + -48 & 255) < 10) goto B_oa;
      n = e;
      if (e < d) goto B_ca;
      label B_pa:
      if (h < l) goto B_x;
      if (eqz(l)) goto B_ra;
      if (d > l) goto B_sa;
      if (d == l) goto B_ra;
      goto B_x;
      label B_sa:
      if ((c + l)[0]:byte < -64) goto B_x;
      label B_ra:
      if (eqz(g)) goto B_ta;
      if (d > h) goto B_ua;
      if (eqz(k)) goto B_x;
      goto B_ta;
      label B_ua:
      if (f[0]:byte <= -65) goto B_x;
      label B_ta:
      f_ed(b + 16, c + l, h - l);
      if (b[16]:ubyte) goto B_ca;
      if (n < m) goto B_y;
      k = b[5]:int;
      if (eqz(m)) goto B_va;
      if (d > m) goto B_wa;
      if (d == m) goto B_va;
      goto B_y;
      label B_wa:
      if ((c + m)[0]:byte < -64) goto B_y;
      label B_va:
      if (eqz(e)) goto B_xa;
      if (eqz(i)) goto B_y;
      label B_xa:
      f_ed(b + 16, c + m, n - m);
      if (b[16]:ubyte) goto B_ca;
      h = b[5]:int;
      e = 0;
      if (d >= j) goto B_ya;
      j = d;
      goto B_aa;
      label B_ya:
      if (eqz(j)) goto B_za;
      if (d > j) goto B_ab;
      if (d == j) goto B_za;
      goto B_z;
      label B_ab:
      if ((c + j)[0]:byte < -64) goto B_z;
      label B_za:
      (a + 8)[0]:int = j;
      goto B_aa;
      label B_oa:
      e = e + 1;
      continue L_la;
    }
    label B_ga:
    g = g + 1;
    continue L_da;
  }
  label B_ca:
  e = 1;
  label B_ba:
  j = d;
  label B_aa:
  g = a.b;
  if (g <= j) goto B_db;
  if (j) goto B_eb;
  f_mi(c, g, 1);
  c = 1;
  goto B_db;
  label B_eb:
  c = f_ph(c, g, 1, j);
  if (eqz(c)) goto B_cb;
  label B_db:
  g = f_wh(20, 4);
  if (eqz(g)) goto B_bb;
  g.c = j;
  g.b = c;
  g.a = 0;
  g.e = select_if(0, h, e);
  g.d = select_if(0, k, e);
  g_a = b + 80;
  return g;
  label B_cb:
  f_mj(j, 1);
  unreachable;
  label B_bb:
  f_mj(20, 4);
  unreachable;
  label B_z:
  f_rf(1055052, 48, 1055100);
  unreachable;
  label B_y:
  f_eg(c, d, m, n, 1055932);
  unreachable;
  label B_x:
  f_eg(c, d, l, h, 1055916);
  unreachable;
  label B_w:
  f_eg(c, d, e, d, 1055900);
  unreachable;
  label B_v:
  f_eg(c, d, h, d, 1055884);
  unreachable;
  label B_u:
  f_eg(c, d, g, d, 1055868);
  unreachable;
  label B_a:
  f_eg(i, h, 0, g, 1055116);
  return unreachable;
}

function f_o(a:int, b:int_ptr, c:int) {
  var f:int;
  var g:int;
  var h:int;
  var i:int;
  var j:int;
  var e:int;
  var m:int;
  var l:int;
  var u:long;
  var t:long;
  var s:long;
  var n:int;
  var o:int;
  var k:int;
  var p:int;
  var d:int = g_a - 32;
  g_a = d;
  e = b[3];
  f = e + 1;
  if (f >= e) goto B_b;
  f_mf(d, 1);
  s = d[0]:long;
  a[0]:int = 1;
  a[1]:long@4 = s;
  goto B_a;
  label B_b:
  if (
    f <= (i = select_if(g = b[0], ((h = g + 1) >> 3) * 7, g < 8)) >> 1) goto B_e;
  f_gb(d + 8, 20, 8, select_if(f, j = i + 1, f > j));
  if (d[2]:int) goto B_g;
  k = (d + 20)[0]:int - e;
  t = d[3]:long@4;
  l = i32_wrap_i64(t >> 32L);
  f = b + 4;
  m = f[0]:int;
  n = i32_wrap_i64(t);
  if (h) goto B_f;
  b[2] = k;
  b[0] = n;
  a[0]:int = 0;
  f[0]:int = l;
  goto B_c;
  label B_g:
  a[1]:long@4 = d[3]:long@4;
  a[0]:int = 1;
  goto B_a;
  label B_f:
  o = m + -20;
  p = 0;
  loop L_h {
    if ((m + p)[0]:byte < 0) goto B_i;
    s = 
      (l + (j = i32_wrap_i64((u = f_za(c, o + (0 - p) * 20)) & t)))[0]:long@1 &
      -9187201950435737472L;
    if (s != 0L) goto B_j;
    f = 8;
    loop L_k {
      j = j + f;
      f = f + 8;
      s = (l + (j = j & n))[0]:long@1 & -9187201950435737472L;
      if (eqz(s)) continue L_k;
    }
    label B_j:
    if (
      (l + (f = (i32_wrap_i64(ctz(s)) >> 3) + j & n))[0]:byte <= -1) goto B_l;
    f = i32_wrap_i64(ctz(l[0]:long & -9187201950435737472L)) >> 3;
    label B_l:
    (l + f)[0]:byte = (j = i32_wrap_i64(u) >> 25);
    ((f + -8 & n) + l + 8)[0]:byte = j;
    f = l + (f ^ -1) * 20;
    (f + 16)[0]:int@1 = ((j = m + (p ^ -1) * 20) + 16)[0]:int@1;
    (f + 8)[0]:long@1 = (j + 8)[0]:long@1;
    f[0]:long@1 = j[0]:long@1;
    label B_i:
    f = p == g;
    p = p + 1;
    if (f) goto B_d;
    continue L_h;
  }
  label B_e:
  n = (b + 4)[0]:int;
  j = 0;
  f = 0;
  loop L_m {
    if (j & 1) goto B_p;
    if (f >= h) goto B_o;
    goto B_n;
    label B_p:
    j = f + 7;
    if (j < f) goto B_o;
    f = j;
    if (j < h) goto B_n;
    label B_o:
    if (h < 8) goto B_s;
    (n + h)[0]:long@1 = n[0]:long@1;
    goto B_r;
    label B_s:
    f_ek(n + 8, n, h);
    if (eqz(h)) goto B_q;
    label B_r:
    var q:int = n + -20;
    f = 0;
    loop L_t {
      var r:int = n + (o = f);
      if (r[0]:ubyte != 128) goto B_u;
      k = q + (0 - o) * 20;
      l = n + (o ^ -1) * 20;
      loop L_w {
        p = g & (h = i32_wrap_i64(f_za(c, k)));
        j = p;
        s = (n + p)[0]:long@1 & -9187201950435737472L;
        if (s != 0L) goto B_x;
        f = 8;
        j = p;
        loop L_y {
          j = j + f;
          f = f + 8;
          s = (n + (j = j & g))[0]:long@1 & -9187201950435737472L;
          if (eqz(s)) continue L_y;
        }
        label B_x:
        if (
          (n + (j = (i32_wrap_i64(ctz(s)) >> 3) + j & g))[0]:byte <= -1) goto B_z;
        j = i32_wrap_i64(ctz(n[0]:long & -9187201950435737472L)) >> 3;
        label B_z:
        if (((j - p ^ o - p) & g) < 8) goto B_aa;
        f = n + (j ^ -1) * 20;
        p = n + j;
        m = p[0]:ubyte;
        p[0]:byte = (h = h >> 25);
        ((j + -8 & g) + n + 8)[0]:byte = h;
        if (m == 255) goto B_v;
        j = f[0]:int@1;
        f[0]:int@1 = l[0]:int@1;
        l[0]:int@1 = j;
        j = l[4]:int@1;
        l[4]:int@1 = f[4]:int@1;
        f[4]:int@1 = j;
        j = f[8]:ushort@1;
        f[8]:short@1 = l[8]:ushort@1;
        l[8]:short@1 = j;
        j = f[10]:ushort@1;
        f[10]:short@1 = l[10]:ushort@1;
        l[10]:short@1 = j;
        j = l[12]:ushort@1;
        l[12]:short@1 = f[12]:ushort@1;
        f[12]:short@1 = j;
        j = l[14]:ushort@1;
        l[14]:short@1 = f[14]:ushort@1;
        f[14]:short@1 = j;
        j = f[16]:ushort@1;
        f[16]:byte = l[16]:ubyte;
        p = l[17]:ubyte;
        l[16]:short@1 = j;
        f[17]:byte = p;
        j = l[18]:ubyte;
        l[18]:byte = f[18]:ubyte;
        f[18]:byte = j;
        j = l[19]:ubyte;
        l[19]:byte = f[19]:ubyte;
        f[19]:byte = j;
        continue L_w;
        label B_aa:
      }
      r[0]:byte = (f = h >> 25);
      ((o + -8 & g) + n + 8)[0]:byte = f;
      goto B_u;
      label B_v:
      r[0]:byte = 255;
      ((o + -8 & g) + n + 8)[0]:byte = 255;
      (f + 16)[0]:int@1 = (l + 16)[0]:int@1;
      (f + 8)[0]:long@1 = (l + 8)[0]:long@1;
      f[0]:long@1 = l[0]:long@1;
      label B_u:
      f = o + 1;
      if (o != g) continue L_t;
    }
    label B_q:
    a[0]:int = 0;
    b[2] = i - e;
    goto B_a;
    label B_n:
    j = n + f;
    j[0]:long =
      (((s = j[0]:long) >> 7L ^ -1L) & 72340172838076673L) +
      (s | 9187201950435737471L);
    j = 1;
    f = f + 1;
    continue L_m;
  }
  label B_d:
  b[2] = k;
  b[0] = n;
  a[0]:int = 0;
  (b + 4)[0]:int = l;
  if (eqz(g)) goto B_a;
  label B_c:
  j = g + (f = i32_wrap_i64(i64_extend_i32_u(h) * 20L) + 7 & -8) + 9;
  if (eqz(j)) goto B_a;
  f_mi(m - f, j, 8);
  label B_a:
  g_a = d + 32;
}

function f_p(a:{ a:int, b:int }, b:int, c:int) {
  var h:int;
  var i:int;
  var j:int;
  var f:long_ptr;
  var e:int;
  var g:int;
  var d:int = g_a - 144;
  g_a = d;
  f_ma(d + 32, b, c);
  if (d[8]:int) goto B_g;
  f_fb(d + 32, b, c);
  if (d[8]:int) goto B_f;
  (d + 24)[0]:long = (d + 64)[0]:long;
  (d + 16)[0]:long = (d + 32 + 24)[0]:long;
  (d + 8)[0]:long = (d + 32 + 16)[0]:long;
  d[0]:long = (d + 32 + 8)[0]:long;
  e = f_wh(12, 4);
  if (eqz(e)) goto B_e;
  f = f_wh(130, 1);
  if (eqz(f)) goto B_d;
  e[1]:int = 130;
  e[0]:int = f;
  f_dk(f, 1049684, 130);
  e[2]:int = 130;
  d[29]:long@4 = 4294967297L;
  d[28]:int = e;
  e = f_wh(128, 1);
  if (eqz(e)) goto B_c;
  d[9]:long@4 = 128L;
  d[8]:int = e;
  d[22]:int = d + 32;
  e = f_v(d + 88, d + 112);
  if (e) goto B_h;
  var l:long = d[9]:long@4;
  g = d[8]:int;
  e = d[28]:int;
  f = d[30]:int;
  if (eqz(f)) goto B_i;
  f = f * 12;
  loop L_j {
    h = (e + 4)[0]:int;
    if (eqz(h)) goto B_k;
    f_mi(e[0]:int, h, 1);
    label B_k:
    e = e + 12;
    f = f + -12;
    if (f) continue L_j;
  }
  e = d[28]:int;
  label B_i:
  f = d[29]:int;
  if (eqz(f)) goto B_l;
  f = i32_wrap_i64(i64_extend_i32_u(f) * 12L);
  if (eqz(f)) goto B_l;
  f_mi(e, f, 4);
  label B_l:
  d[9]:long@4 = l;
  d[8]:int = g;
  f_lb(d + 112, d, 1049872, 10, d + 32);
  e = d[28]:int;
  if (eqz(e)) goto B_m;
  f = d[29]:int;
  if (eqz(f)) goto B_m;
  f_mi(e, f, 1);
  label B_m:
  (d + 32 + 24)[0]:long = (d + 24)[0]:long;
  e = d + 32 + 16;
  e[0]:long = (d + 16)[0]:long;
  (d + 32 + 8)[0]:long = (d + 8)[0]:long;
  d[4]:long = d[0]:long;
  f_ha(d + 112, d + 32);
  i = e[0]:int;
  if (eqz(i)) goto B_a;
  if ((d + 60)[0]:int) goto B_n;
  j = i + 1;
  goto B_b;
  label B_n:
  f = (d + 32 + 20)[0]:int;
  e = f + 8;
  h = f + (j = i + 1);
  var m:long = (f[0] ^ -1L) & -9187201950435737472L;
  loop L_o {
    if (m == 0L) goto B_q;
    l = m;
    goto B_p;
    label B_q:
    loop L_r {
      if (e >= h) goto B_b;
      f = f + -160;
      l = e[0]:long;
      g = e + 8;
      e = g;
      l = l & -9187201950435737472L;
      if (l == -9187201950435737472L) continue L_r;
    }
    l = l ^ -9187201950435737472L;
    e = g;
    label B_p:
    m = l + -1L & l;
    g = f + (0 - (i32_wrap_i64(ctz(l)) >> 3)) * 20;
    var k:int = (g + -8)[0]:int;
    if (eqz(k)) continue L_o;
    f_mi((g + -12)[0]:int, k, 1);
    continue L_o;
  }
  label B_h:
  f_wg(d + 32);
  d[18]:int = e;
  (d + 52)[0]:int = 1;
  d[9]:long@4 = 1L;
  d[8]:int = 1049848;
  d[23]:int = 10;
  d[12]:int = d + 88;
  d[22]:int = d + 72;
  f_ag(d + 32, 1049856);
  unreachable;
  label B_g:
  d[11]:long@4 = d[9]:long@4;
  d[10]:int = c;
  d[9]:int = c;
  d[8]:int = b;
  f_ee(1049532, 43, d + 32, 1049576, 1049652);
  unreachable;
  label B_f:
  d[28]:int = d[9]:int;
  f_ee(1049532, 43, d + 112, 1049592, 1049668);
  unreachable;
  label B_e:
  f_mj(12, 4);
  unreachable;
  label B_d:
  f_mj(130, 1);
  unreachable;
  label B_c:
  f_mj(128, 1);
  unreachable;
  label B_b:
  f = i + (e = i32_wrap_i64(i64_extend_i32_u(j) * 20L) + 7 & -8) + 9;
  if (eqz(f)) goto B_a;
  f_mi((d + 52)[0]:int - e, f, 8);
  label B_a:
  if (d[28]:int) goto B_u;
  (d + 88 + 16)[0]:long = (d + 136)[0]:long;
  (d + 88 + 8)[0]:long = (d + 112 + 16)[0]:long;
  d[11]:long = (d + 112 + 8)[0]:long;
  d[20]:int = 0;
  d[9]:long = 1L;
  f_fg(d + 32, d + 72, 1049344);
  if (f_c(d + 88, d + 32)) goto B_t;
  br_table[B_v, B_v, B_v, B_x, B_w, ..B_y](d[88]:ubyte)
  label B_y:
  f_oa(d + 88 | 4);
  goto B_v;
  label B_x:
  e = (d + 96)[0]:int;
  if (eqz(e)) goto B_v;
  f_mi(d[23]:int, e, 1);
  goto B_v;
  label B_w:
  f_wd(d + 88 | 4);
  e = (d + 96)[0]:int;
  if (eqz(e)) goto B_v;
  e = i32_wrap_i64(i64_extend_i32_u(e) * 24L);
  if (eqz(e)) goto B_v;
  f_mi(d[23]:int, e, 8);
  label B_v:
  e = d[20]:int;
  if (e < 0) goto B_ba;
  h = d[18]:int;
  if (e) goto B_aa;
  f = 1;
  goto B_z;
  label B_ba:
  f_zf();
  unreachable;
  label B_aa:
  f = f_wh(e, 1);
  if (eqz(f)) goto B_s;
  f_dk(f, h, e);
  label B_z:
  g = d[19]:int;
  if (eqz(g)) goto B_ca;
  f_mi(h, g, 1);
  label B_ca:
  if (eqz(c)) goto B_da;
  f_mi(b, c, 1);
  label B_da:
  a.b = e;
  a.a = f;
  g_a = d + 144;
  return ;
  label B_u:
  d[8]:int = d[29]:int;
  f_ee(1049532, 43, d + 32, 1049592, 1049884);
  unreachable;
  label B_t:
  f_ee(1049368, 55, d + 112, 1049516, 1049500);
  unreachable;
  label B_s:
  f_mj(e, 1);
  unreachable;
}

function f_q(a:{ a:int, b:int }, b:int):int {
  var h:int;
  var d:int_ptr;
  var c:int;
  var g:int;
  var e:int_ptr;
  var f:int_ptr;
  var i:int;
  if (b > 1279) goto B_b;
  c = b >> 5;
  d = a.a;
  if (eqz(d)) goto B_f;
  e = a + (d << 2);
  f = a + (d + c << 2);
  d = d + -1;
  g = d > 39;
  loop L_g {
    if (g) goto B_c;
    h = c + d;
    if (h >= 40) goto B_e;
    f[0] = e[0];
    f = f + -4;
    e = e + -4;
    d = d + -1;
    if (d != -1) continue L_g;
  }
  label B_f:
  if (b < 32) goto B_a;
  a.b = 0;
  if (b >= 64) goto B_d;
  goto B_a;
  label B_e:
  f_ne(h, 40, 1075496);
  unreachable;
  label B_d:
  (a + 8)[0]:int = 0;
  d = select_if(c, 1, c > 1);
  if (d == 2) goto B_a;
  (a + 12)[0]:int = 0;
  if (d == 3) goto B_a;
  (a + 16)[0]:int = 0;
  if (d == 4) goto B_a;
  (a + 20)[0]:int = 0;
  if (d == 5) goto B_a;
  (a + 24)[0]:int = 0;
  if (d == 6) goto B_a;
  (a + 28)[0]:int = 0;
  if (d == 7) goto B_a;
  (a + 32)[0]:int = 0;
  if (d == 8) goto B_a;
  (a + 36)[0]:int = 0;
  if (d == 9) goto B_a;
  (a + 40)[0]:int = 0;
  if (d == 10) goto B_a;
  (a + 44)[0]:int = 0;
  if (d == 11) goto B_a;
  (a + 48)[0]:int = 0;
  if (d == 12) goto B_a;
  (a + 52)[0]:int = 0;
  if (d == 13) goto B_a;
  (a + 56)[0]:int = 0;
  if (d == 14) goto B_a;
  (a + 60)[0]:int = 0;
  if (d == 15) goto B_a;
  (a + 64)[0]:int = 0;
  if (d == 16) goto B_a;
  (a + 68)[0]:int = 0;
  if (d == 17) goto B_a;
  (a + 72)[0]:int = 0;
  if (d == 18) goto B_a;
  (a + 76)[0]:int = 0;
  if (d == 19) goto B_a;
  (a + 80)[0]:int = 0;
  if (d == 20) goto B_a;
  (a + 84)[0]:int = 0;
  if (d == 21) goto B_a;
  (a + 88)[0]:int = 0;
  if (d == 22) goto B_a;
  (a + 92)[0]:int = 0;
  if (d == 23) goto B_a;
  (a + 96)[0]:int = 0;
  if (d == 24) goto B_a;
  (a + 100)[0]:int = 0;
  if (d == 25) goto B_a;
  (a + 104)[0]:int = 0;
  if (d == 26) goto B_a;
  (a + 108)[0]:int = 0;
  if (d == 27) goto B_a;
  (a + 112)[0]:int = 0;
  if (d == 28) goto B_a;
  (a + 116)[0]:int = 0;
  if (d == 29) goto B_a;
  (a + 120)[0]:int = 0;
  if (d == 30) goto B_a;
  (a + 124)[0]:int = 0;
  if (d == 31) goto B_a;
  (a + 128)[0]:int = 0;
  if (d == 32) goto B_a;
  (a + 132)[0]:int = 0;
  if (d == 33) goto B_a;
  (a + 136)[0]:int = 0;
  if (d == 34) goto B_a;
  (a + 140)[0]:int = 0;
  if (d == 35) goto B_a;
  (a + 144)[0]:int = 0;
  if (d == 36) goto B_a;
  (a + 148)[0]:int = 0;
  if (d == 37) goto B_a;
  (a + 152)[0]:int = 0;
  if (d == 38) goto B_a;
  (a + 156)[0]:int = 0;
  if (d == 39) goto B_a;
  (a + 160)[0]:int = 0;
  if (d == 40) goto B_a;
  f_ne(40, 40, 1075496);
  unreachable;
  label B_c:
  f_ne(d, 40, 1075496);
  unreachable;
  label B_b:
  f_rf(1075538, 29, 1075496);
  unreachable;
  label B_a:
  e = a.a + c;
  g = b & 31;
  if (g) goto B_h;
  a.a = e;
  return a;
  label B_h:
  d = e + -1;
  if (d > 39) goto B_j;
  i = e;
  f = (a + (d << 2) + 4)[0]:int;
  d = f >> (b = 0 - b);
  if (eqz(d)) goto B_i;
  if (e > 39) goto B_k;
  (a + (e << 2) + 4)[0]:int = d;
  i = e + 1;
  goto B_i;
  label B_k:
  f_ne(e, 40, 1075496);
  unreachable;
  label B_j:
  f_ne(d, 40, 1075496);
  unreachable;
  label B_i:
  h = c + 1;
  if (h >= e) goto B_m;
  b = b & 31;
  d = (e << 2) + a + -4;
  loop L_n {
    if (e + -2 >= 40) goto B_l;
    (d + 4)[0]:int = f << g | (f = d[0]) >> b;
    d = d + -4;
    if (h < (e = e + -1)) continue L_n;
  }
  label B_m:
  d = a + (c << 2) + 4;
  d[0] = d[0] << g;
  a.a = i;
  return a;
  label B_l:
  f_ne(-1, 40, 1075496);
  return unreachable;
}

function f_r(a:{ a:int, b:int }, b:int_ptr, c:int_ptr) {
  var h:int;
  var g:int;
  var l:int;
  var k:ubyte_ptr;
  var i:int;
  var m:int;
  var j:int_ptr;
  var d:int_ptr = g_a - 16;
  g_a = d;
  var e:int_ptr = b + 4;
  var f:int_ptr = b + 8;
  loop L_k {
    g = f[0];
    if (g >= (h = e[0])) goto B_n;
    i = 1;
    j = g + 1;
    k = b[0];
    l = g;
    loop L_o {
      m = (k + l)[0]:ubyte;
      if ((m + 1053412)[0]:ubyte) goto B_l;
      f[0] = (l = l + 1);
      i = i + 1;
      j = j + 1;
      if (h != l) continue L_o;
    }
    g = h;
    goto B_m;
    label B_n:
    if (g != h) goto B_h;
    k = b[0];
    label B_m:
    e = 1;
    l = 0;
    i = 1;
    if (eqz(g)) goto B_p;
    j = g & 3;
    if (g + -1 >= 3) goto B_r;
    l = 0;
    i = 1;
    goto B_q;
    label B_r:
    f = g & -4;
    i = 1;
    l = 0;
    loop L_s {
      l = 
        select_if(0,
                  select_if(1,
                            select_if(2,
                                      select_if(3, l + 4, h = k[0] == 10),
                                      m = (k + 1)[0]:ubyte == 10),
                            g = (k + 2)[0]:ubyte == 10),
                  c = (k + 3)[0]:ubyte == 10);
      i = i + h + m + g + c;
      k = k + 4;
      f = f + -4;
      if (f) continue L_s;
    }
    label B_q:
    if (eqz(j)) goto B_p;
    loop L_t {
      l = select_if(0, l + 1, f = k[0] == 10);
      k = k + 1;
      i = i + f;
      j = j + -1;
      if (j) continue L_t;
    }
    label B_p:
    d[0] = 4;
    a.b = f_tf(d, i, l);
    goto B_i;
    label B_l:
    if (m == 92) goto B_u;
    if (m == 34) goto B_j;
    i = 1;
    (b + 8)[0]:int = (f = l + 1);
    if (l >= h) goto B_g;
    b = f & 3;
    if (l >= 3) goto B_w;
    l = 0;
    goto B_v;
    label B_w:
    f = f & -4;
    i = 1;
    l = 0;
    loop L_x {
      l = 
        select_if(0,
                  select_if(1,
                            select_if(2,
                                      select_if(3, l + 4, h = k[0] == 10),
                                      m = (k + 1)[0]:ubyte == 10),
                            g = (k + 2)[0]:ubyte == 10),
                  c = (k + 3)[0]:ubyte == 10);
      i = i + h + m + g + c;
      k = k + 4;
      f = f + -4;
      if (f) continue L_x;
    }
    label B_v:
    if (eqz(b)) goto B_y;
    j = j & 3;
    loop L_z {
      l = select_if(0, l + 1, f = k[0] == 10);
      k = k + 1;
      i = i + f;
      j = j + -1;
      if (j) continue L_z;
    }
    label B_y:
    d[0] = 15;
    a.b = f_tf(d, i, l);
    e = 1;
    goto B_i;
    label B_u:
    if (l < g) goto B_f;
    if (l > h) goto B_e;
    m = k + g;
    if ((c + 4)[0]:int - (k = (j = c + 8)[0]) >= (h = i + -1)) goto B_aa;
    f_jd(c, k, h);
    k = j[0];
    label B_aa:
    f_dk(c[0] + k, m, h);
    f[0] = l + 1;
    j[0] = k + i + -1;
    l = f_k(b, 1, c);
    if (eqz(l)) continue L_k;
  }
  a.b = l;
  e = 1;
  goto B_i;
  label B_j:
  j = (c + 8)[0]:int;
  if (eqz(j)) goto B_ba;
  if (l < g) goto B_d;
  if (l > h) goto B_c;
  f = k + g;
  if ((c + 4)[0]:int - j >= (k = i + -1)) goto B_ca;
  f_jd(c, j, k);
  j = (c + 8)[0]:int;
  label B_ca:
  f_dk(c[0] + j, f, k);
  (b + 8)[0]:int = l + 1;
  (c + 8)[0]:int = (l = j + i + -1);
  (a + 12)[0]:int = l;
  a.b = 1;
  (a + 8)[0]:int = c[0];
  e = 0;
  goto B_i;
  label B_ba:
  if (l < g) goto B_b;
  if (l > h) goto B_a;
  e = 0;
  a.b = 0;
  (a + 12)[0]:int = i + -1;
  (a + 8)[0]:int = k + g;
  (b + 8)[0]:int = l + 1;
  label B_i:
  a.a = e;
  g_a = d + 16;
  return ;
  label B_h:
  f_ne(g, h, 1053332);
  unreachable;
  label B_g:
  f_sj(f, h, 1053316);
  unreachable;
  label B_f:
  f_tj(g, l, 1053348);
  unreachable;
  label B_e:
  f_sj(l, h, 1053348);
  unreachable;
  label B_d:
  f_tj(g, l, 1053380);
  unreachable;
  label B_c:
  f_sj(l, h, 1053380);
  unreachable;
  label B_b:
  f_tj(g, l, 1053364);
  unreachable;
  label B_a:
  f_sj(l, h, 1053364);
  unreachable;
}

function f_s(a:byte_ptr, b:int, c:{ a:byte, b:ubyte, c:ubyte, d:ubyte }, d:int) {
  var f:int;
  var g:int;
  var h:int;
  var i:byte_ptr;
  var e:int = g_a - 112;
  g_a = e;
  e[3]:int = d;
  e[2]:int = c;
  if (b < 257) goto B_h;
  f = 256;
  if (a[256] > -65) goto B_i;
  f = 255;
  if (a[255] > -65) goto B_i;
  f = 254;
  if (a[254] > -65) goto B_i;
  f = 253;
  label B_i:
  if (f < b) goto B_g;
  if (f != b) goto B_e;
  label B_h:
  e[5]:int = b;
  e[4]:int = a;
  f = 0;
  g = 1072168;
  goto B_f;
  label B_g:
  e[5]:int = f;
  e[4]:int = a;
  f = 5;
  g = 1073643;
  label B_f:
  e[7]:int = f;
  e[6]:int = g;
  f = c > b;
  if (f) goto B_d;
  if (d > b) goto B_d;
  if (c > d) goto B_j;
  if (eqz(c)) goto B_l;
  if (c < b) goto B_m;
  if (b == c) goto B_l;
  goto B_k;
  label B_m:
  if ((a + c)[0]:byte < -64) goto B_k;
  label B_l:
  c = d;
  label B_k:
  e[8]:int = c;
  d = b;
  if (c >= b) goto B_n;
  f = c + 1;
  if (f < (d = select_if(0, d = c + -3, d > c))) goto B_c;
  if (d == f) goto B_o;
  f = a + f - (h = a + d);
  i = a + c;
  if (i[0] <= -65) goto B_p;
  g = f + -1;
  goto B_o;
  label B_p:
  if (d == c) goto B_o;
  c = i + -1;
  if (c.a <= -65) goto B_q;
  g = f + -2;
  goto B_o;
  label B_q:
  if (h == c) goto B_o;
  c = i + -2;
  if (c.a <= -65) goto B_r;
  g = f + -3;
  goto B_o;
  label B_r:
  if (h == c) goto B_o;
  c = i + -3;
  if (c.a <= -65) goto B_s;
  g = f + -4;
  goto B_o;
  label B_s:
  if (h == c) goto B_o;
  g = f + -5;
  label B_o:
  d = g + d;
  label B_n:
  if (eqz(d)) goto B_t;
  if (d < b) goto B_u;
  if (d == b) goto B_t;
  goto B_a;
  label B_u:
  if ((a + d)[0]:byte <= -65) goto B_a;
  label B_t:
  if (d == b) goto B_b;
  c = a + d;
  b = c.a;
  if (b > -1) goto B_y;
  a = c.b & 63;
  f = b & 31;
  if (b > -33) goto B_x;
  c = f << 6 | a;
  goto B_w;
  label B_y:
  e[9]:int = b & 255;
  b = 1;
  goto B_v;
  label B_x:
  a = a << 6 | (c.c & 63);
  if (b >= -16) goto B_z;
  c = a | f << 12;
  goto B_w;
  label B_z:
  c = (a << 6 | (c.d & 63)) | (f << 18 & 1835008);
  if (c == 1114112) goto B_b;
  label B_w:
  e[9]:int = c;
  b = 1;
  if (c < 128) goto B_v;
  b = 2;
  if (c < 2048) goto B_v;
  b = select_if(3, 4, c < 65536);
  label B_v:
  e[10]:int = d;
  e[11]:int = b + d;
  (e + 48 + 20)[0]:int = 5;
  (e + 108)[0]:int = 69;
  (e + 100)[0]:int = 69;
  (e + 72 + 20)[0]:int = 70;
  (e + 84)[0]:int = 71;
  e[13]:long@4 = 5L;
  e[12]:int = 1073876;
  e[19]:int = 29;
  e[16]:int = e + 72;
  e[26]:int = e + 24;
  e[24]:int = e + 16;
  e[22]:int = e + 40;
  e[20]:int = e + 36;
  e[18]:int = e + 32;
  f_ag(e + 48, 1073916);
  unreachable;
  label B_j:
  (e + 100)[0]:int = 69;
  (e + 72 + 20)[0]:int = 69;
  (e + 84)[0]:int = 29;
  (e + 48 + 20)[0]:int = 4;
  e[13]:long@4 = 4L;
  e[12]:int = 1073760;
  e[19]:int = 29;
  e[16]:int = e + 72;
  e[24]:int = e + 24;
  e[22]:int = e + 16;
  e[20]:int = e + 12;
  e[18]:int = e + 8;
  f_ag(e + 48, 1073792);
  unreachable;
  label B_e:
  f_eg(a, b, 0, f, e);
  unreachable;
  label B_d:
  e[10]:int = select_if(c, d, f);
  (e + 48 + 20)[0]:int = 3;
  (e + 72 + 20)[0]:int = 69;
  (e + 84)[0]:int = 69;
  e[13]:long@4 = 3L;
  e[12]:int = 1073684;
  e[19]:int = 29;
  e[16]:int = e + 72;
  e[22]:int = e + 24;
  e[20]:int = e + 16;
  e[18]:int = e + 40;
  f_ag(e + 48, 1073708);
  unreachable;
  label B_c:
  f_tj(d, f, e);
  unreachable;
  label B_b:
  f_rf(1072240, 43, 1073808);
  unreachable;
  label B_a:
  f_eg(a, b, d, b, e);
  unreachable;
}

function f(a:int_ptr, b:int, c:int):int {
  var f:int;
  var h:int;
  var j:int;
  var g:int;
  var i:int_ptr;
  var e:int;
  var d:int = g_a - 48;
  g_a = d;
  e = (a + 8)[0]:int;
  if (e >= (f = (a + 4)[0]:int)) goto B_o;
  g = a[0];
  h = (g + e)[0]:ubyte;
  br_table[B_l, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_k, ..B_q](
    h + -34);
  label B_q:
  br_table[B_m, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_s, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_u, B_p, B_p, B_p, B_p, B_p, B_t, B_p, B_p, B_p, B_p, B_p, B_p, B_r, ..B_p](
    h + -91)
  label B_u:
  i = a + 8;
  i[0] = (h = e + 1);
  if (h >= f) goto B_b;
  i[0] = (j = e + 2);
  if ((g + h)[0]:ubyte != 117) goto B_v;
  if (j == (h = select_if(f, h, f > h))) goto B_b;
  (a + 8)[0]:int = (f = e + 3);
  if ((g + j)[0]:ubyte != 108) goto B_v;
  if (f == h) goto B_b;
  (a + 8)[0]:int = e + 4;
  if ((g + f)[0]:ubyte == 108) goto B_g;
  label B_v:
  d[2]:int = 9;
  a = f_ef(a, d + 8);
  goto B_a;
  label B_t:
  i = a + 8;
  i[0] = (h = e + 1);
  if (h >= f) goto B_c;
  i[0] = (j = e + 2);
  if ((g + h)[0]:ubyte != 114) goto B_w;
  if (j == (h = select_if(f, h, f > h))) goto B_c;
  (a + 8)[0]:int = (f = e + 3);
  if ((g + j)[0]:ubyte != 117) goto B_w;
  if (f == h) goto B_c;
  (a + 8)[0]:int = e + 4;
  if ((g + f)[0]:ubyte == 101) goto B_h;
  label B_w:
  d[2]:int = 9;
  a = f_ef(a, d + 8);
  goto B_a;
  label B_s:
  i = a + 8;
  i[0] = (h = e + 1);
  if (h >= f) goto B_d;
  i[0] = (j = e + 2);
  if ((g + h)[0]:ubyte != 97) goto B_x;
  if (j == (h = select_if(f, h, f > h))) goto B_d;
  (a + 8)[0]:int = (f = e + 3);
  if ((g + j)[0]:ubyte != 108) goto B_x;
  if (f == h) goto B_d;
  (a + 8)[0]:int = (i = e + 4);
  if ((g + f)[0]:ubyte != 115) goto B_x;
  if (i == h) goto B_d;
  (a + 8)[0]:int = e + 5;
  if ((g + i)[0]:ubyte == 101) goto B_i;
  label B_x:
  d[2]:int = 9;
  a = f_ef(a, d + 8);
  goto B_a;
  label B_r:
  d[8]:byte = 11;
  a = f_gf(f_od(d + 8, b, c), a);
  goto B_a;
  label B_p:
  if ((h + -48 & 255) < 10) goto B_n;
  label B_o:
  d[2]:int = 10;
  a = f_gf(f_df(a, d + 8), a);
  goto B_a;
  label B_n:
  f_ga(d + 8, a, 1);
  if (d[2]:int) goto B_j;
  (d + 40)[0]:long = (d + 24)[0]:long;
  d[4]:long = d[2]:long;
  a = f_gf(f_ie(d + 32, b, c), a);
  goto B_a;
  label B_m:
  d[8]:byte = 10;
  a = f_gf(f_od(d + 8, b, c), a);
  goto B_a;
  label B_l:
  (a + 20)[0]:int = 0;
  (a + 8)[0]:int = e + 1;
  f_r(d + 32, a, a + 12);
  if (eqz(d[8]:int)) goto B_e;
  a = d[9]:int;
  goto B_a;
  label B_k:
  (a + 8)[0]:int = e + 1;
  f_ga(d + 8, a, 0);
  if (eqz(d[2]:int)) goto B_f;
  label B_j:
  a = d[3]:int;
  goto B_a;
  label B_i:
  d[4]:short = 0;
  a = f_gf(f_od(d + 8, b, c), a);
  goto B_a;
  label B_h:
  d[4]:short = 256;
  a = f_gf(f_od(d + 8, b, c), a);
  goto B_a;
  label B_g:
  d[8]:byte = 7;
  a = f_gf(f_od(d + 8, b, c), a);
  goto B_a;
  label B_f:
  (d + 32 + 8)[0]:long = (d + 24)[0]:long;
  d[4]:long = d[2]:long;
  a = f_gf(f_ie(d + 32, b, c), a);
  goto B_a;
  label B_e:
  d[3]:long@4 = (d + 32 + 8)[0]:long;
  d[8]:byte = 5;
  a = f_gf(f_od(d + 8, b, c), a);
  goto B_a;
  label B_d:
  d[2]:int = 5;
  a = f_ef(a, d + 8);
  goto B_a;
  label B_c:
  d[2]:int = 5;
  a = f_ef(a, d + 8);
  goto B_a;
  label B_b:
  d[2]:int = 5;
  a = f_ef(a, d + 8);
  label B_a:
  g_a = d + 48;
  return a;
}

function f_u(a:int, b:int):int {
  var h:int;
  var i:int;
  var k:int;
  var c:{ a:byte, b:byte, c:byte }
  c = a + 3 & -4;
  var d:int = c - a;
  if (d > b) goto B_b;
  if (d > 4) goto B_b;
  var e:int = b - d;
  if (e < 4) goto B_b;
  var f:int = e & 3;
  var g:int = 0;
  b = 0;
  if (eqz(d)) goto B_c;
  h = d & 3;
  if (c + (a ^ -1) >= 3) goto B_e;
  b = 0;
  c = a;
  goto B_d;
  label B_e:
  i = d & -4;
  b = 0;
  c = a;
  loop L_f {
    b = 
      b + (c.a > -65) + ((c + 1)[0]:byte > -65) + ((c + 2)[0]:byte > -65) +
      ((c + 3)[0]:byte > -65);
    c = c + 4;
    i = i + -4;
    if (i) continue L_f;
  }
  label B_d:
  if (eqz(h)) goto B_c;
  loop L_g {
    b = b + (c.a > -65);
    c = c + 1;
    h = h + -1;
    if (h) continue L_g;
  }
  label B_c:
  a = a + d;
  if (eqz(f)) goto B_h;
  c = a + (e & -4);
  g = c.a > -65;
  if (f == 1) goto B_h;
  g = g + (c.b > -65);
  if (f == 2) goto B_h;
  g = g + (c.c > -65);
  label B_h:
  d = e >> 2;
  i = g + b;
  loop L_i {
    g = a;
    if (eqz(d)) goto B_a;
    e = select_if(d, 192, d < 192);
    f = e & 3;
    var j:int = e << 2;
    k = e & 252;
    a = k << 2;
    if (a) goto B_k;
    c = 0;
    goto B_j;
    label B_k:
    h = g + a;
    c = 0;
    a = g;
    loop L_l {
      b = (a + 12)[0]:int;
      c = (((b ^ -1) >> 7 | b >> 6) & 16843009) +
          ((((b = (a + 8)[0]:int) ^ -1) >> 7 | b >> 6) & 16843009) +
          ((((b = (a + 4)[0]:int) ^ -1) >> 7 | b >> 6) & 16843009) +
          ((((b = a[0]:int) ^ -1) >> 7 | b >> 6) & 16843009) + c;
      a = a + 16;
      if (a != h) continue L_l;
    }
    label B_j:
    a = g + j;
    d = d - e;
    i = (((c >> 8 & 16711935) + (c & 16711935)) * 65537 >> 16) + i;
    if (eqz(f)) continue L_i;
  }
  a = g + (k << 2);
  e = f + 1073741823;
  c = e & 1073741823;
  b = c + 1;
  d = b & 3;
  if (c >= 3) goto B_n;
  c = 0;
  goto B_m;
  label B_n:
  b = b & 2147483644;
  c = 0;
  loop L_o {
    h = (a + 12)[0]:int;
    c = (((h ^ -1) >> 7 | h >> 6) & 16843009) +
        ((((h = (a + 8)[0]:int) ^ -1) >> 7 | h >> 6) & 16843009) +
        ((((h = (a + 4)[0]:int) ^ -1) >> 7 | h >> 6) & 16843009) +
        ((((h = a[0]:int) ^ -1) >> 7 | h >> 6) & 16843009) + c;
    a = a + 16;
    b = b + -4;
    if (b) continue L_o;
  }
  label B_m:
  if (eqz(d)) goto B_p;
  b = e + -1073741823;
  loop L_q {
    h = a[0]:int;
    c = (((h ^ -1) >> 7 | h >> 6) & 16843009) + c;
    a = a + 4;
    b = b + -1;
    if (b) continue L_q;
  }
  label B_p:
  return (((c >> 8 & 16711935) + (c & 16711935)) * 65537 >> 16) + i;
  label B_b:
  if (b) goto B_r;
  return 0;
  label B_r:
  c = b & 3;
  if (b + -1 >= 3) goto B_t;
  i = 0;
  goto B_s;
  label B_t:
  b = b & -4;
  i = 0;
  loop L_u {
    i = i + (a[0]:byte > -65) + ((a + 1)[0]:byte > -65) +
        ((a + 2)[0]:byte > -65) +
        ((a + 3)[0]:byte > -65);
    a = a + 4;
    b = b + -4;
    if (b) continue L_u;
  }
  label B_s:
  if (eqz(c)) goto B_a;
  loop L_v {
    i = i + (a[0]:byte > -65);
    a = a + 1;
    c = c + -1;
    if (c) continue L_v;
  }
  label B_a:
  return i;
}

function f_v(a:int_ptr, b:int_ptr):int {
  var e:int_ptr;
  var k:int;
  var q:ushort_ptr@1;
  var s:int;
  var r:int;
  var c:int = b[2];
  var d:int_ptr = b[0];
  e = a[0];
  if ((e + 4)[0]:int != (b = (a = e + 8)[0])) goto B_a;
  f_hd(e, b, 1);
  b = a[0];
  label B_a:
  (e[0] + b)[0]:byte = 91;
  a[0] = (b = b + 1);
  if (c) goto B_d;
  if ((e + 4)[0]:int != b) goto B_b;
  f_hd(e, b, 1);
  goto B_c;
  label B_d:
  var f:int = d + c * 12;
  c = c != 0;
  var g:int_ptr = e + 4;
  var h:int_ptr = e + 8;
  loop L_e {
    if (c & 1) goto B_f;
    if (g[0] != b) goto B_g;
    f_hd(e, b, 1);
    b = h[0];
    label B_g:
    (e[0] + b)[0]:byte = 44;
    h[0] = (b = b + 1);
    label B_f:
    var i:int = d[2];
    var j:int = d[0];
    if (g[0] != b) goto B_h;
    f_hd(e, b, 1);
    b = h[0];
    label B_h:
    d = d + 12;
    (e[0] + b)[0]:byte = 34;
    h[0] = (k = b + 1);
    var l:int = j + -1;
    var m:int = i ^ -1;
    var n:int = j + i;
    var o:int = 0;
    a = j;
    var p:int = 0;
    loop L_i {
      b = 0;
      loop L_n {
        c = a + b;
        if (c != n) goto B_o;
        if (i == p) goto B_l;
        if (eqz(p)) goto B_m;
        if (i <= p) goto B_p;
        if ((j + p)[0]:byte > -65) goto B_m;
        label B_p:
        f_eg(j, i, p, i, 1050208);
        unreachable;
        label B_o:
        b = b + 1;
        q = c[0]:ubyte;
        c = (q + 1054464)[0]:ubyte;
        if (eqz(c)) continue L_n;
      }
      r = o + b;
      s = r + -1;
      if (s <= p) goto B_j;
      if (eqz(p)) goto B_q;
      if (i > p) goto B_r;
      if (i == p) goto B_q;
      goto B_k;
      label B_r:
      if ((j + p)[0]:byte < -64) goto B_k;
      label B_q:
      if (s < i) goto B_t;
      s = i;
      if (m + o + b) goto B_k;
      goto B_s;
      label B_t:
      if ((l + o + b)[0]:byte <= -65) goto B_k;
      label B_s:
      o = j + p;
      if (g[0] - k >= (p = s - p)) goto B_u;
      f_hd(e, k, p);
      k = h[0];
      label B_u:
      f_dk(e[0] + k, o, p);
      h[0] = (k = k + p);
      goto B_j;
      label B_m:
      c = j + p;
      if (g[0] - k >= (b = i - p)) goto B_v;
      f_hd(e, k, b);
      k = h[0];
      label B_v:
      f_dk(e[0] + k, c, b);
      h[0] = (k = k + b);
      label B_l:
      if (g[0] != k) goto B_w;
      f_hd(e, k, 1);
      k = h[0];
      label B_w:
      (e[0] + k)[0]:byte = 34;
      h[0] = (b = k + 1);
      c = 0;
      if (d != f) continue L_e;
      if ((e + 4)[0]:int != b) goto B_b;
      f_hd(e, b, 1);
      goto B_c;
      label B_k:
      f_eg(j, i, p, o + b + -1, 1050192);
      unreachable;
      label B_j:
      br_table[B_z, B_ga, B_ga, B_ga, B_ga, B_ga, B_fa, B_ga, B_ga, B_ga, B_ea, B_ga, B_ga, B_ga, B_ga, B_ga, B_ga, B_ga, B_da, B_ga, B_ga, B_ga, B_ca, B_ga, B_ba, B_aa, ..B_ha](
        c + -92)
      label B_ha:
      q = 1050236;
      if (c == 34) goto B_y;
      label B_ga:
      f_rf(1050044, 40, 1050176);
      unreachable;
      label B_fa:
      q = 1050232;
      goto B_y;
      label B_ea:
      q = 1050230;
      goto B_y;
      label B_da:
      q = 1050228;
      goto B_y;
      label B_ca:
      q = 1050226;
      goto B_y;
      label B_ba:
      q = 1050224;
      goto B_y;
      label B_aa:
      p = ((q & 15) + 1054400)[0]:ubyte;
      q = ((q >> 4) + 1054400)[0]:ubyte;
      if (g[0] - k > 5) goto B_ia;
      f_hd(e, k, 6);
      k = h[0];
      label B_ia:
      c = e[0] + k;
      c[5]:byte = p;
      c[4]:byte = q;
      c[0]:int@1 = 808482140;
      k = k + 6;
      goto B_x;
      label B_z:
      q = 1050234;
      label B_y:
      if (g[0] - k > 1) goto B_ja;
      f_hd(e, k, 2);
      k = h[0];
      label B_ja:
      (e[0] + k)[0]:short@1 = q[0];
      k = k + 2;
      label B_x:
      a = a + b;
      h[0] = k;
      p = s + 1;
      o = r;
      continue L_i;
    }
  }
  label B_c:
  b = (e + 8)[0]:int;
  label B_b:
  (e[0] + b)[0]:byte = 93;
  (e + 8)[0]:int = b + 1;
  return 0;
}

function f_w(a:double, b:int):int {
  var h:int;
  var g:int;
  var f:int;
  var c:{ a:long, b:int } = g_a - 16;
  g_a = c;
  var i:long = i64_reinterpret_f64(a);
  var d:byte_ptr = i32_wrap_i64(i >> 52L) & 2047;
  var e:int = 0;
  if (i >= 0L) goto B_a;
  b[0]:byte = 45;
  e = 1;
  label B_a:
  var j:long = i & 4503599627370495L;
  if (d) goto B_m;
  if (eqz(j)) goto B_l;
  label B_m:
  f_l(c, j, d);
  d = 17;
  f = c.b;
  i = c.a;
  if (i > 9999999999999999L) goto B_n;
  d = 16;
  if (i > 999999999999999L) goto B_n;
  d = 15;
  if (i > 99999999999999L) goto B_n;
  d = 14;
  if (i > 9999999999999L) goto B_n;
  d = 13;
  if (i > 999999999999L) goto B_n;
  d = 12;
  if (i > 99999999999L) goto B_n;
  d = 11;
  if (i > 9999999999L) goto B_n;
  d = 10;
  if (i > 999999999L) goto B_n;
  d = 9;
  if (i > 99999999L) goto B_n;
  d = 8;
  if (i > 9999999L) goto B_n;
  d = 7;
  if (i > 999999L) goto B_n;
  d = 6;
  if (i > 99999L) goto B_n;
  d = 5;
  if (i > 9999L) goto B_n;
  d = 4;
  if (i > 999L) goto B_n;
  d = 3;
  if (i > 99L) goto B_n;
  d = select_if(2, 1, i > 9L);
  label B_n:
  g = d + f;
  if (f < 0) goto B_o;
  if (g < 17) goto B_k;
  label B_o:
  f = g + -1;
  if (f < 16) goto B_j;
  if (g + 4 < 5) goto B_i;
  if (d != 1) goto B_f;
  d = b + e;
  (d + 1)[0]:byte = 101;
  d[0] = i32_wrap_i64(i) + 48;
  e = b + (d = e | 2);
  if (f < 0) goto B_h;
  b = f;
  goto B_g;
  label B_l:
  b = b + e;
  b[0]:short@1 = d_calledOptionunwraponaNoneval[7552]:ushort@1;
  (b + 2)[0]:byte = d_calledOptionunwraponaNoneval[7554]:ubyte;
  d = i32_wrap_i64(i >> 63L) + 3;
  goto B_b;
  label B_k:
  f_ib(i, h = b + d + e);
  if (d >= g) goto B_p;
  f_bk(h, 48, f);
  label B_p:
  (b + (e = g + e))[0]:short@1 = 12334;
  d = e + 2;
  goto B_b;
  label B_j:
  f_ib(i, b + (d = d + (f = e + 1)));
  f_ek(b + e, b + f, g);
  (b + g + e)[0]:byte = 46;
  goto B_b;
  label B_i:
  h = b + e;
  h[0]:short@1 = 11824;
  f = 2 - g;
  if (g > -1) goto B_q;
  f_bk(h + 2, 48, select_if(f, 3, f > 3) + -2);
  label B_q:
  f_ib(i, b + (d = d + e + f));
  goto B_b;
  label B_h:
  e[0]:byte = 45;
  b = 1 - g;
  e = e + 1;
  label B_g:
  if (b > 99) goto B_e;
  if (b > 9) goto B_r;
  e[0]:byte = b + 48;
  d = (f >> 31) + 1 + d;
  goto B_b;
  label B_r:
  e[0]:short@1 = ((b << 1) + 1066824)[0]:ushort@1;
  d = (f >> 31 | 2) + d;
  goto B_b;
  label B_f:
  f_ib(i, h = (d = d + e) + b + 1);
  e = b + e;
  e[0]:byte = (e = e + 1)[0]:ubyte;
  e[0]:byte = 46;
  h[0]:byte = 101;
  e = b + (d = d + 2);
  if (f < 0) goto B_d;
  b = f;
  goto B_c;
  label B_e:
  e[0]:byte = (g = b / 100) + 48;
  e[1]:short@1 = ((b - g * 100 << 1) + 1066824)[0]:ushort@1;
  d = (f >> 31) + 3 + d;
  goto B_b;
  label B_d:
  e[0]:byte = 45;
  b = 1 - g;
  e = e + 1;
  label B_c:
  if (b > 99) goto B_s;
  if (b > 9) goto B_t;
  e[0]:byte = b + 48;
  d = (f >> 31) + 1 + d;
  goto B_b;
  label B_t:
  e[0]:short@1 = ((b << 1) + 1066824)[0]:ushort@1;
  d = (f >> 31 | 2) + d;
  goto B_b;
  label B_s:
  e[0]:byte = (g = b / 100) + 48;
  e[1]:short@1 = ((b - g * 100 << 1) + 1066824)[0]:ushort@1;
  d = (f >> 31) + 3 + d;
  label B_b:
  g_a = c + 16;
  return d;
}

function f_x(a:{ a:int, b:int, c:int, d:int }) {
  var b:int_ptr;
  var f:int_ptr;
  var d:int;
  var e:int_ptr;
  a = f_nk(a);
  var c:int_ptr = f_kk(a, b = f_ej(a));
  if (f_fj(a)) goto B_c;
  d = a.a;
  if (f_pi(a)) goto B_e;
  b = d + b;
  a = f_lk(a, d);
  if (a != 0[269236]:int) goto B_d;
  if ((c[1] & 3) != 3) goto B_c;
  0[269234]:int = b;
  f_sg(a, b, c);
  return ;
  label B_e:
  if (eqz(f_fl(1076536, a - d, a = d + b + 16))) goto B_b;
  0[269238]:int = 0[269238]:int - a;
  return ;
  label B_d:
  if (d < 256) goto B_f;
  f_nc(a);
  goto B_c;
  label B_f:
  e = (a + 12)[0]:int;
  if (e == (f = (a + 8)[0]:int)) goto B_g;
  f[3] = e;
  e[2] = f;
  goto B_c;
  label B_g:
  0[269134]:int = 0[269134]:int & -2 << (d >> 3);
  label B_c:
  if (eqz(f_ei(c))) goto B_i;
  f_sg(a, b, c);
  goto B_h;
  label B_i:
  if (c == 0[269237]:int) goto B_m;
  if (c != 0[269236]:int) goto B_l;
  0[269236]:int = a;
  0[269234]:int = (b = 0[269234]:int + b);
  f_gh(a, b);
  return ;
  label B_m:
  0[269237]:int = a;
  0[269235]:int = (b = 0[269235]:int + b);
  a.b = b | 1;
  if (a == 0[269236]:int) goto B_k;
  goto B_j;
  label B_l:
  d = f_ej(c);
  b = d + b;
  if (d < 256) goto B_o;
  f_nc(c);
  goto B_n;
  label B_o:
  e = (c + 12)[0]:int;
  if (e == (c = (c + 8)[0]:int)) goto B_p;
  c[3] = e;
  e[2] = c;
  goto B_n;
  label B_p:
  0[269134]:int = 0[269134]:int & -2 << (d >> 3);
  label B_n:
  f_gh(a, b);
  if (a != 0[269236]:int) goto B_h;
  0[269234]:int = b;
  goto B_b;
  label B_k:
  0[269234]:int = 0;
  0[269236]:int = 0;
  label B_j:
  if (0[269244]:int >= b) goto B_b;
  a = f_bl();
  a = (a - f_nh(a, 8) + f_nh(20, 8) + f_nh(16, 8) + -65544 & -9) + -3;
  if (eqz(select_if(a, b = 0 - (f_nh(16, 8) << 2), b > a))) goto B_b;
  if (eqz(0[269237]:int)) goto B_b;
  a = f_bl();
  b = f_nh(a, 8);
  d = f_nh(20, 8);
  e = f_nh(16, 8);
  c = 0;
  f = 0[269235]:int;
  if (f <= (a = e + d + b - a)) goto B_q;
  d = f + (a ^ -1) & -65536;
  b = 0[269237]:int;
  a = 1076960;
  loop L_s {
    if (a.a > b) goto B_t;
    if (f_ri(a) > b) goto B_r;
    label B_t:
    a = a.c;
    if (a) continue L_s;
  }
  a = 0;
  label B_r:
  c = 0;
  if (f_gj(a)) goto B_q;
  if (eqz(f_gl(1076536, (a + 12)[0]:int >> 1))) goto B_q;
  if (a.b < d) goto B_q;
  b = 1076960;
  loop L_u {
    if (f_pg(a, b)) goto B_q;
    b = b[2];
    if (b) continue L_u;
  }
  if (eqz(f_el(1076536, a.a, b = a.b, b - d))) goto B_q;
  if (eqz(d)) goto B_q;
  a.b = a.b - d;
  0[269238]:int = 0[269238]:int - d;
  b = 0[269235]:int;
  a = 0[269237]:int;
  0[269237]:int = (a = f_kk(a, c = f_nh(c = f_mk(a), 8) - c));
  0[269235]:int = (b = b - d + c);
  a.b = b | 1;
  c = f_bl();
  e = f_nh(c, 8);
  f = f_nh(20, 8);
  var g:int = f_nh(16, 8);
  f_kk(a, b)[1]:int = g + f + e - c;
  0[269244]:int = 2097152;
  c = d;
  label B_q:
  if (c != 0 - f_rc()) goto B_b;
  if (0[269235]:int <= 0[269244]:int) goto B_b;
  0[269244]:int = -1;
  return ;
  label B_h:
  if (b < 256) goto B_a;
  f_lc(a, b);
  0[269246]:int = (a = 0[269246]:int + -1);
  if (a) goto B_b;
  f_rc();
  return ;
  label B_b:
  return ;
  label B_a:
  c = b >> 3;
  b = (c << 3) + 1076544;
  d = 0[269134]:int;
  if (eqz(d & (c = 1 << c))) goto B_w;
  c = b[2];
  goto B_v;
  label B_w:
  0[269134]:int = d | c;
  c = b;
  label B_v:
  b[2] = a;
  c[3] = a;
  a.d = b;
  a.c = c;
}

function f_y(a:int_ptr, b:int, c:int, d:int, e:int) {
  var f:int;
  var q:long;
  var h:int;
  var i:int;
  var j:int;
  var k:int;
  var o:long;
  var p:long;
  var r:long;
  var l:int;
  var s:long;
  var n:long = b[0]:long;
  if (eqz(n)) goto B_g;
  if (n > 2305843009213693951L) goto B_f;
  if (eqz(d)) goto B_d;
  b = 
    (((-96 -
       (f = 
          select_if(
            (b = 
               select_if(
                 (b = 
                    select_if(
                      (b = 
                         select_if(
                           (b = select_if((b = b[12]:ushort) + -32, b, f = n < 4294967296L)) +
                           -16,
                           b,
                           f = (n = select_if(n << 32L, n, f)) < 281474976710656L)) +
                      -8,
                      b,
                      f = (n = select_if(n << 16L, n, f)) < 72057594037927936L)) +
                 -4,
                 b,
                 f = (n = select_if(n << 8L, n, f)) < 1152921504606846976L)) +
            -2,
            b,
            f = (n = select_if(n << 4L, n, f)) < 4611686018427387904L) +
          (i32_wrap_i64((n = select_if(n << 2L, n, f)) >> 63L) ^ -1)) <<
       16) >>
      16) *
     80 +
     86960) /
    2126;
  if (b >= 81) goto B_e;
  b = b << 4;
  var g:int = (b + 1070242)[0]:ushort;
  o = (b + 1070232)[0]:long;
  p = o & 4294967295L;
  r = p * (q = (n = n << ((n ^ -1L) >> 63L)) >> 32L);
  n = 
    (r >> 32L) + (o = o >> 32L) * q +
    ((o = o * (n = n & 4294967295L)) >> 32L) +
    ((r & 4294967295L) + (p * n >> 32L) + (o & 4294967295L) + 2147483648L >>
     32L);
  h = 
    i32_wrap_i64(
      n >>
      (q = i64_extend_i32_u((b = -64 - f + (b + 1070240)[0]:ushort) & 63)));
  if (h < 10000) goto B_k;
  if (h < 1000000) goto B_j;
  if (h < 100000000) goto B_i;
  i = select_if(8, 9, f = h < 1000000000);
  f = select_if(100000000, 1000000000, f);
  goto B_h;
  label B_k:
  if (h < 100) goto B_l;
  i = select_if(2, 3, f = h < 1000);
  f = select_if(100, 1000, f);
  goto B_h;
  label B_l:
  f = select_if(1, 10, h < 10);
  i = h > 9;
  goto B_h;
  label B_j:
  i = select_if(4, 5, f = h < 100000);
  f = select_if(10000, 100000, f);
  goto B_h;
  label B_i:
  i = select_if(6, 7, f = h < 10000000);
  f = select_if(1000000, 10000000, f);
  label B_h:
  s = 1L << q;
  j = (i - g << 16) + 65536 >> 16;
  if (j <= (g = (e << 16) >> 16)) goto B_n;
  o = n & (r = s + -1L);
  k = b & 65535;
  l = select_if((j - e << 16) >> 16, d, j - g < d);
  var m:int = l + -1;
  b = 0;
  loop L_o {
    g = h / f;
    if (d == b) goto B_c;
    h = h - g * f;
    (c + b)[0]:byte = g + 48;
    if (m == b) goto B_b;
    if (i == b) goto B_m;
    b = b + 1;
    g = f < 10;
    f = f / 10;
    if (eqz(g)) continue L_o;
  }
  f_rf(1071776, 25, 1071956);
  unreachable;
  label B_n:
  f_eb(a, c, d, 0, j, e, n / 10L, i64_extend_i32_u(f) << q, s);
  return ;
  label B_m:
  b = b + 1;
  f = select_if(b, d, b > d);
  var t:long = i64_extend_i32_u(k + -1 & 63);
  n = 1L;
  loop L_p {
    if (eqz(n >> t)) goto B_q;
    a[0] = 0;
    return ;
    label B_q:
    if (f == b) goto B_a;
    n = n * 10L;
    p = o * 10L;
    o = p & r;
    (c + b)[0]:byte = i32_wrap_i64(p >> q) + 48;
    if (l != (b = b + 1)) continue L_p;
  }
  f_eb(a, c, d, l, j, e, o, s, n);
  return ;
  label B_g:
  f_rf(1069739, 28, 1071872);
  unreachable;
  label B_f:
  f_rf(1071888, 36, 1071924);
  unreachable;
  label B_e:
  f_ne(b, 81, 1071576);
  unreachable;
  label B_d:
  f_rf(1071836, 33, 1071940);
  unreachable;
  label B_c:
  f_ne(d, d, 1071972);
  unreachable;
  label B_b:
  f_eb(a, c, d, l, j, e, (i64_extend_i32_u(h) << q) + o, i64_extend_i32_u(f) << q, s);
  return ;
  label B_a:
  f_ne(f, d, 1071988);
  unreachable;
}

function f_z(a:{ a:ubyte, b:ubyte }, b:int):int {
  var c:int = g_a - 48;
  g_a = c;
  br_table[B_s, B_r, B_q, B_p, B_o, B_n, B_m, B_l, B_k, B_j, B_i, B_h, B_g, B_f, B_e, B_d, B_c, B_b, ..B_s](
    a.a)
  label B_s:
  c[8]:byte = a.b;
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 2L;
  c[6]:int = 1067356;
  c[5]:int = 43;
  c[10]:int = c + 16;
  c[4]:int = c + 8;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_r:
  c[1]:long = (a + 8)[0]:long;
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 2L;
  c[6]:int = 1067328;
  c[5]:int = 44;
  c[10]:int = c + 16;
  c[4]:int = c + 8;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_q:
  c[1]:long = (a + 8)[0]:long;
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 2L;
  c[6]:int = 1067328;
  c[5]:int = 45;
  c[10]:int = c + 16;
  c[4]:int = c + 8;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_p:
  c[1]:double = (a + 8)[0]:double;
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 2L;
  c[6]:int = 1067300;
  c[5]:int = 46;
  c[10]:int = c + 16;
  c[4]:int = c + 8;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_o:
  c[2]:int = (a + 4)[0]:int;
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 2L;
  c[6]:int = 1067268;
  c[5]:int = 47;
  c[10]:int = c + 16;
  c[4]:int = c + 8;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_n:
  c[1]:long = (a + 4)[0]:long@4;
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 1L;
  c[6]:int = 1067248;
  c[5]:int = 48;
  c[10]:int = c + 16;
  c[4]:int = c + 8;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_m:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067232;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_l:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067212;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_k:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067192;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_j:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067172;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_i:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067148;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_h:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067132;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_g:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067120;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_f:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067108;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_e:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067088;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_d:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067064;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_c:
  (c + 44)[0]:int = 0;
  c[10]:int = 1067024;
  c[7]:long@4 = 1L;
  c[6]:int = 1067040;
  a = f_pe(b, c + 24);
  goto B_a;
  label B_b:
  a = f_ih(b, (a + 4)[0]:int, (a + 8)[0]:int);
  label B_a:
  g_a = c + 48;
  return a;
}

function f_aa(a:int, b:int, c:int):int {
  var i:{ a:byte, b:ubyte, c:ubyte, d:ubyte }
  var f:int;
  var h:int_ptr;
  var g:int;
  var e:int;
  var d:{ a:byte, b:ubyte, c:ubyte, d:ubyte } = a[4]:int;
  e = a[2]:int;
  if (e == 1) goto B_f;
  if (d != 1) goto B_e;
  label B_f:
  if (d != 1) goto B_b;
  f = b + c;
  g = (a + 20)[0]:int;
  if (g) goto B_d;
  h = 0;
  i = b;
  goto B_c;
  label B_e:
  d = call_indirect(a[6]:int, b, c, ((a + 28)[0]:int)[3]:int);
  goto B_a;
  label B_d:
  h = 0;
  i = b;
  loop L_g {
    d = i;
    if (d == f) goto B_b;
    i = d.a;
    if (i <= -1) goto B_i;
    i = d + 1;
    goto B_h;
    label B_i:
    if (i >= -32) goto B_j;
    i = d + 2;
    goto B_h;
    label B_j:
    if (i >= -16) goto B_k;
    i = d + 3;
    goto B_h;
    label B_k:
    if (((((d.c & 63) << 6 | (d.b & 63) << 12) | (d.d & 63)) |
         ((i & 255) << 18 & 1835008)) ==
        1114112) goto B_b;
    i = d + 4;
    label B_h:
    h = h - d + i;
    g = g + -1;
    if (g) continue L_g;
  }
  label B_c:
  if (i == f) goto B_b;
  d = i.a;
  if (d > -1) goto B_l;
  if (d < -32) goto B_l;
  if (d < -16) goto B_l;
  if (((((i.c & 63) << 6 | (i.b & 63) << 12) | (i.d & 63)) |
       ((d & 255) << 18 & 1835008)) ==
      1114112) goto B_b;
  label B_l:
  if (h) goto B_o;
  i = 0;
  goto B_n;
  label B_o:
  if (h < c) goto B_p;
  d = 0;
  i = c;
  if (h == c) goto B_n;
  goto B_m;
  label B_p:
  d = 0;
  i = h;
  if ((b + h)[0]:byte < -64) goto B_m;
  label B_n:
  h = i;
  d = b;
  label B_m:
  c = select_if(h, c, d);
  b = select_if(d, b, d);
  label B_b:
  if (e) goto B_q;
  return call_indirect(a[6]:int, b, c, ((a + 28)[0]:int)[3]:int);
  label B_q:
  f = (a + 12)[0]:int;
  if (c < 16) goto B_s;
  i = f_u(b, c);
  goto B_r;
  label B_s:
  if (c) goto B_t;
  i = 0;
  goto B_r;
  label B_t:
  h = c & 3;
  if (c + -1 >= 3) goto B_v;
  i = 0;
  d = b;
  goto B_u;
  label B_v:
  g = c & -4;
  i = 0;
  d = b;
  loop L_w {
    i = 
      i + (d.a > -65) + ((d + 1)[0]:byte > -65) + ((d + 2)[0]:byte > -65) +
      ((d + 3)[0]:byte > -65);
    d = d + 4;
    g = g + -4;
    if (g) continue L_w;
  }
  label B_u:
  if (eqz(h)) goto B_r;
  loop L_x {
    i = i + (d.a > -65);
    d = d + 1;
    h = h + -1;
    if (h) continue L_x;
  }
  label B_r:
  if (f <= i) goto B_y;
  d = 0;
  h = f - i;
  g = h;
  br_table[B_z, B_ba, B_aa, ..B_z](
    select_if(0, i = a[32]:ubyte, i == 3) & 3)
  label B_ba:
  g = 0;
  d = h;
  goto B_z;
  label B_aa:
  d = h >> 1;
  g = h + 1 >> 1;
  label B_z:
  d = d + 1;
  h = (a + 28)[0]:int;
  i = a[1]:int;
  a = a[6]:int;
  loop L_da {
    d = d + -1;
    if (eqz(d)) goto B_ca;
    if (eqz(call_indirect(a, i, h[4]))) continue L_da;
  }
  return 1;
  label B_ca:
  d = 1;
  if (i == 1114112) goto B_a;
  if (call_indirect(a, b, c, h[3])) goto B_a;
  d = 0;
  loop L_ea {
    if (g != d) goto B_fa;
    return g < g;
    label B_fa:
    d = d + 1;
    if (eqz(call_indirect(a, i, h[4]))) continue L_ea;
  }
  return d + -1 < g;
  label B_y:
  return call_indirect(a[6]:int, b, c, ((a + 28)[0]:int)[3]:int);
  label B_a:
  return d;
}

function f_ba(a:int, b:int, c:int_ptr):int {
  var g:int_ptr;
  var h:int;
  var n:int;
  var i:int;
  var p:int;
  var o:int;
  var r:int;
  var d:{ a:int, b:int, c:int, d:ubyte } = g_a - 16;
  g_a = d;
  var e:int = 1;
  var f:int = c[6];
  if (call_indirect(f, 34, h = (g = (c + 28)[0]:int)[4])) goto B_b;
  if (b) goto B_d;
  i = 0;
  goto B_c;
  label B_d:
  var j:int = a + b;
  var k:{ a:byte, b:ubyte, c:ubyte, d:ubyte } = a;
  i = 0;
  var l:int = 0;
  loop L_g {
    var m:int = k;
    c = k.a;
    if (c <= -1) goto B_i;
    k = k + 1;
    n = c & 255;
    goto B_h;
    label B_i:
    o = k.b & 63;
    p = c & 31;
    if (c > -33) goto B_j;
    n = p << 6 | o;
    k = k + 2;
    goto B_h;
    label B_j:
    o = o << 6 | (k.c & 63);
    if (c >= -16) goto B_k;
    n = o | p << 12;
    k = k + 3;
    goto B_h;
    label B_k:
    n = (o << 6 | (k.d & 63)) | (p << 18 & 1835008);
    if (n == 1114112) goto B_e;
    k = k + 4;
    label B_h:
    f_sb(d, n, 65537);
    c = d.a;
    br_table[B_n, B_m, B_n, B_o, ..B_n](c);
    label B_o:
    if (d.c + d.d == 1) goto B_m;
    label B_n:
    if (l < i) goto B_f;
    if (eqz(i)) goto B_p;
    if (i < b) goto B_q;
    if (i == b) goto B_p;
    goto B_f;
    label B_q:
    if ((a + i)[0]:byte < -64) goto B_f;
    label B_p:
    if (eqz(l)) goto B_r;
    if (l < b) goto B_s;
    if (l != b) goto B_f;
    goto B_r;
    label B_s:
    if ((a + l)[0]:byte <= -65) goto B_f;
    label B_r:
    if (call_indirect(f, a + i, l - i, g[3])) goto B_l;
    o = d.d;
    var q:int = d.c;
    r = d.b;
    if (r != 1114112) goto B_u;
    loop L_v {
      p = c;
      c = 1;
      i = 92;
      br_table[B_t, B_t, B_w, B_x, ..B_t](p)
      label B_x:
      p = o & 255;
      c = 3;
      o = 0;
      i = 125;
      br_table[B_t, B_w, B_y, B_z, B_aa, B_ba, ..B_t](p)
      label B_ba:
      o = 4;
      i = 92;
      goto B_w;
      label B_aa:
      o = 3;
      i = 117;
      c = 3;
      goto B_w;
      label B_z:
      o = 2;
      i = 123;
      goto B_w;
      label B_y:
      o = select_if(2, 1, q);
      i = (1114112 >> (q << 2) & 1) | 48;
      q = select_if(q + -1, 0, q);
      label B_w:
      if (eqz(call_indirect(f, i, h))) continue L_v;
      goto B_l;
    }
    label B_u:
    loop L_ca {
      p = c;
      c = 1;
      i = 92;
      br_table[B_t, B_fa, B_da, B_ga, ..B_t](p)
      label B_ga:
      p = o & 255;
      c = 3;
      o = 0;
      i = 125;
      br_table[B_t, B_da, B_ea, B_ja, B_ia, B_ha, ..B_t](p)
      label B_ja:
      o = 2;
      i = 123;
      goto B_da;
      label B_ia:
      o = 3;
      i = 117;
      c = 3;
      goto B_da;
      label B_ha:
      o = 4;
      i = 92;
      goto B_da;
      label B_fa:
      c = 0;
      i = r;
      goto B_da;
      label B_ea:
      o = select_if(2, 1, q);
      i = select_if(48, 87, (i = r >> (q << 2) & 15) < 10) + i;
      q = select_if(q + -1, 0, q);
      label B_da:
      if (call_indirect(f, i, h)) goto B_l;
      continue L_ca;
    }
    label B_t:
    c = 1;
    if (n < 128) goto B_ka;
    c = 2;
    if (n < 2048) goto B_ka;
    c = select_if(3, 4, n < 65536);
    label B_ka:
    i = c + l;
    label B_m:
    l = l - m + k;
    if (k != j) continue L_g;
    goto B_e;
    label B_l:
  }
  e = 1;
  goto B_b;
  label B_f:
  f_eg(a, b, i, l, c);
  unreachable;
  label B_e:
  if (i) goto B_la;
  i = 0;
  goto B_c;
  label B_la:
  if (i < b) goto B_ma;
  if (i == b) goto B_c;
  goto B_a;
  label B_ma:
  if ((a + i)[0]:byte <= -65) goto B_a;
  label B_c:
  if (call_indirect(f, a + i, b - i, g[3])) goto B_b;
  e = call_indirect(f, 34, h);
  label B_b:
  g_a = d + 16;
  return e;
  label B_a:
  f_eg(a, b, i, b, c);
  return unreachable;
}

function f_ca(a:int, b:int, c:int, d:int, e:int, f:int):int {
  var g:int;
  var i:int;
  var h:int;
  var j:int_ptr;
  var k:int;
  if (eqz(b)) goto B_b;
  h = select_if(43, 1114112, g = (b = a[0]:int) & 1);
  i = g + f;
  goto B_a;
  label B_b:
  i = f + 1;
  b = a[0]:int;
  h = 45;
  label B_a:
  if (b & 4) goto B_d;
  c = 0;
  goto B_c;
  label B_d:
  if (d < 16) goto B_f;
  g = f_u(c, d);
  goto B_e;
  label B_f:
  if (d) goto B_g;
  g = 0;
  goto B_e;
  label B_g:
  j = d & 3;
  if (d + -1 >= 3) goto B_i;
  g = 0;
  b = c;
  goto B_h;
  label B_i:
  k = d & -4;
  g = 0;
  b = c;
  loop L_j {
    g = g + (b[0]:byte > -65) + ((b + 1)[0]:byte > -65) +
        ((b + 2)[0]:byte > -65) +
        ((b + 3)[0]:byte > -65);
    b = b + 4;
    k = k + -4;
    if (k) continue L_j;
  }
  label B_h:
  if (eqz(j)) goto B_e;
  loop L_k {
    g = g + (b[0]:byte > -65);
    b = b + 1;
    j = j + -1;
    if (j) continue L_k;
  }
  label B_e:
  i = g + i;
  label B_c:
  if (a[2]:int) goto B_m;
  b = 1;
  if (f_pf(a, h, c, d)) goto B_l;
  return call_indirect(a[6]:int, e, f, ((a + 28)[0]:int)[3]:int);
  label B_m:
  g = (a + 12)[0]:int;
  if (g <= i) goto B_r;
  if (a[0]:ubyte & 8) goto B_n;
  b = 0;
  j = g - i;
  i = j;
  br_table[B_o, B_q, B_p, ..B_o](
    select_if(1, g = a[32]:ubyte, g == 3) & 3);
  label B_r:
  b = 1;
  if (f_pf(a, h, c, d)) goto B_l;
  return call_indirect(a[6]:int, e, f, ((a + 28)[0]:int)[3]:int);
  label B_q:
  i = 0;
  b = j;
  goto B_o;
  label B_p:
  b = j >> 1;
  i = j + 1 >> 1;
  label B_o:
  b = b + 1;
  j = (a + 28)[0]:int;
  g = a[1]:int;
  k = a[6]:int;
  loop L_t {
    b = b + -1;
    if (eqz(b)) goto B_s;
    if (eqz(call_indirect(k, g, j[4]))) continue L_t;
  }
  return 1;
  label B_s:
  b = 1;
  if (g == 1114112) goto B_l;
  if (f_pf(a, h, c, d)) goto B_l;
  if (call_indirect(a[6]:int, e, f, (a[7]:int)[3]:int)) goto B_l;
  j = a[7]:int;
  a = a[6]:int;
  b = 0;
  loop L_v {
    if (i != b) goto B_w;
    b = i;
    goto B_u;
    label B_w:
    b = b + 1;
    if (eqz(call_indirect(a, g, j[4]))) continue L_v;
  }
  b = b + -1;
  label B_u:
  b = b < i;
  goto B_l;
  label B_n:
  var l:int = a[1]:int;
  a[1]:int = 48;
  var m:int = a[32]:ubyte;
  b = 1;
  a[32]:byte = 1;
  if (f_pf(a, h, c, d)) goto B_l;
  b = 0;
  j = g - i;
  d = j;
  br_table[B_x, B_z, B_y, ..B_x](
    select_if(1, g = a[32]:ubyte, g == 3) & 3)
  label B_z:
  d = 0;
  b = j;
  goto B_x;
  label B_y:
  b = j >> 1;
  d = j + 1 >> 1;
  label B_x:
  b = b + 1;
  j = (a + 28)[0]:int;
  g = a[1]:int;
  k = a[6]:int;
  loop L_ba {
    b = b + -1;
    if (eqz(b)) goto B_aa;
    if (eqz(call_indirect(k, g, j[4]))) continue L_ba;
  }
  return 1;
  label B_aa:
  b = 1;
  if (g == 1114112) goto B_l;
  if (call_indirect(a[6]:int, e, f, (a[7]:int)[3]:int)) goto B_l;
  b = a[7]:int;
  k = a[6]:int;
  j = 0;
  loop L_da {
    if (d == j) goto B_ca;
    j = j + 1;
    if (eqz(call_indirect(k, g, b[4]:int))) continue L_da;
  }
  b = 1;
  if (j + -1 < d) goto B_l;
  label B_ca:
  a[32]:byte = m;
  a[1]:int = l;
  return 0;
  label B_l:
  return b;
}

function f_da(a:int, b:double, c:int, d:int):int {
  var i:long;
  var g:int;
  var h:int;
  var m:long;
  var k:long;
  var f:int;
  var e:int = g_a - 1136;
  g_a = e;
  i = i64_reinterpret_f64(b);
  if (eqz(eqz(i & 9223372036854775807L))) goto B_b;
  f = 4;
  goto B_a;
  label B_b:
  var j:long = i & 4503599627370495L;
  k = select_if(j | 4503599627370496L,
                i << 1L & 9007199254740990L,
                g = i32_wrap_i64(i >> 52L) & 2047);
  var l:long = k & 1L;
  m = i & 9218868437227405312L;
  if (eqz(m)) goto B_d;
  if (m != 9218868437227405312L) goto B_c;
  f = select_if(3, 2, eqz(j));
  goto B_a;
  label B_d:
  g = g + -1075;
  f = i32_wrap_i64(l) ^ 1;
  m = 1L;
  goto B_a;
  label B_c:
  k = 
    select_if(18014398509481984L, k << 1L, h = k == 4503599627370496L);
  m = select_if(2L, 1L, h);
  f = i32_wrap_i64(l) ^ 1;
  g = select_if(-1077, -1076, h) + g;
  label B_a:
  e[564]:short = g;
  e[140]:long = m;
  e[139]:long = 1L;
  e[138]:long = k;
  e[1130]:byte = f;
  if (f != 2) goto B_f;
  c = 1072168;
  h = 0;
  goto B_e;
  label B_f:
  if (c) goto B_g;
  c = select_if(1072163, 1072168, i < 0L);
  h = i32_wrap_i64(i >> 63L);
  goto B_e;
  label B_g:
  c = select_if(1072163, 1072164, i < 0L);
  h = 1;
  label B_e:
  f = f + -2;
  br_table[B_n, B_m, B_k, B_l, ..B_n](select_if(f, 3, f < 3) & 255);
  label B_n:
  e[262]:int = 3;
  e[261]:int = 1072172;
  e[520]:short = 2;
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e + 1040;
  f = 1;
  goto B_h;
  label B_m:
  e[262]:int = 3;
  e[261]:int = 1072169;
  e[520]:short = 2;
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e + 1040;
  f = 1;
  goto B_h;
  label B_l:
  f = select_if(-12, 5, (f = (g << 16) >> 16) < 0) * f;
  if (f > 16063) goto B_j;
  f_y(e + 1040,
      e + 1104,
      e + 16,
      g = (f >> 4) + 21,
      f = select_if(0 - d, -32768, d < 32768));
  f = (f << 16) >> 16;
  if (e[260]:int) goto B_p;
  f_b(e + 1088, e + 1104, e + 16, g, f);
  goto B_o;
  label B_p:
  (e + 1088 + 8)[0]:int = (e + 1040 + 8)[0]:int;
  e[136]:long = e[130]:long;
  label B_o:
  g = e[548]:short;
  if (g <= f) goto B_q;
  f_nb(e + 8, e[272]:int, e[273]:int, g, d, e + 1040, 4);
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e[2]:int;
  f = e[3]:int;
  goto B_h;
  label B_q:
  f = 2;
  e[520]:short = 2;
  if (d) goto B_r;
  f = 1;
  e[262]:int = 1;
  e[261]:int = 1072168;
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e + 1040;
  goto B_h;
  label B_r:
  (e + 1056)[0]:int = d;
  e[526]:short = 0;
  e[262]:int = 2;
  e[261]:int = 1072160;
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e + 1040;
  goto B_h;
  label B_k:
  f = 2;
  e[520]:short = 2;
  if (eqz(d)) goto B_i;
  (e + 1056)[0]:int = d;
  e[526]:short = 0;
  e[262]:int = 2;
  e[261]:int = 1072160;
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e + 1040;
  goto B_h;
  label B_j:
  f_rf(1072175, 37, 1072212);
  unreachable;
  label B_i:
  f = 1;
  e[262]:int = 1;
  e[261]:int = 1072168;
  e[273]:int = h;
  e[272]:int = c;
  e[274]:int = e + 1040;
  label B_h:
  (e + 1100)[0]:int = f;
  f = f_wa(a, e + 1088);
  g_a = e + 1136;
  return f;
}

function f_ea(a:{ a:short, b:short }, b:int_ptr) {
  var g:int;
  var h:int;
  var d:int;
  var e:int;
  var l:int;
  var f:int;
  var i:int;
  var k:int;
  var j:int;
  var c:int_ptr = g_a - 16;
  g_a = c;
  d = b + 8;
  e = d[0]:int;
  f = e + 4;
  if (f > (g = (b + 4)[0]:int)) goto B_e;
  if (g <= e) goto B_c;
  d = b[0];
  (b + 8)[0]:int = (h = e + 1);
  i = ((d + e)[0]:ubyte + 1053684)[0]:ubyte;
  if (i != 255) goto B_d;
  f = h;
  goto B_b;
  label B_e:
  d[0]:int = g;
  j = 1;
  e = 0;
  k = 1;
  if (eqz(g)) goto B_f;
  d = b[0];
  b = g & 3;
  if (g + -1 >= 3) goto B_h;
  e = 0;
  k = 1;
  goto B_g;
  label B_h:
  g = g & -4;
  k = 1;
  e = 0;
  loop L_i {
    e = select_if(
          0,
          select_if(1,
                    select_if(2,
                              select_if(3, e + 4, f = d[0]:ubyte == 10),
                              h = (d + 1)[0]:ubyte == 10),
                    l = (d + 2)[0]:ubyte == 10),
          i = (d + 3)[0]:ubyte == 10);
    k = k + f + h + l + i;
    d = d + 4;
    g = g + -4;
    if (g) continue L_i;
  }
  label B_g:
  if (eqz(b)) goto B_f;
  loop L_j {
    e = select_if(0, e + 1, g = d[0]:ubyte == 10);
    d = d + 1;
    k = k + g;
    b = b + -1;
    if (b) continue L_j;
  }
  label B_f:
  c[0] = 4;
  (a + 4)[0]:int = f_tf(c, k, e);
  goto B_a;
  label B_d:
  k = select_if(0, k = g - e, k > g);
  if (k != 1) goto B_k;
  e = h;
  goto B_c;
  label B_k:
  (b + 8)[0]:int = (l = e + 2);
  h = ((d + h)[0]:ubyte + 1053684)[0]:ubyte;
  if (h != 255) goto B_l;
  f = l;
  goto B_b;
  label B_l:
  if (k != 2) goto B_m;
  e = l;
  goto B_c;
  label B_m:
  (b + 8)[0]:int = (e = e + 3);
  l = ((d + l)[0]:ubyte + 1053684)[0]:ubyte;
  if (l != 255) goto B_n;
  f = e;
  goto B_b;
  label B_n:
  if (k == 3) goto B_c;
  (b + 8)[0]:int = f;
  e = ((d + e)[0]:ubyte + 1053684)[0]:ubyte;
  if (e == 255) goto B_b;
  a.b = (((i << 4) + h << 4) + l << 4) + e;
  j = 0;
  goto B_a;
  label B_c:
  f_ne(e, g, 1053396);
  unreachable;
  label B_b:
  if (f > g) goto B_o;
  j = 1;
  e = 0;
  k = 1;
  if (eqz(f)) goto B_p;
  b = f & 3;
  if (f + -1 >= 3) goto B_r;
  e = 0;
  k = 1;
  goto B_q;
  label B_r:
  g = f & -4;
  k = 1;
  e = 0;
  loop L_s {
    e = select_if(
          0,
          select_if(1,
                    select_if(2,
                              select_if(3, e + 4, f = d[0]:ubyte == 10),
                              h = (d + 1)[0]:ubyte == 10),
                    l = (d + 2)[0]:ubyte == 10),
          i = (d + 3)[0]:ubyte == 10);
    k = k + f + h + l + i;
    d = d + 4;
    g = g + -4;
    if (g) continue L_s;
  }
  label B_q:
  if (eqz(b)) goto B_p;
  loop L_t {
    e = select_if(0, e + 1, g = d[0]:ubyte == 10);
    d = d + 1;
    k = k + g;
    b = b + -1;
    if (b) continue L_t;
  }
  label B_p:
  c[0] = 11;
  (a + 4)[0]:int = f_tf(c, k, e);
  goto B_a;
  label B_o:
  f_sj(f, g, 1053316);
  unreachable;
  label B_a:
  a.a = j;
  g_a = c + 16;
}

function f_fa(a:int, b:{ a:int, b:int }, c:int, d:int):int {
  var e:int;
  var f:int;
  var h:int;
  var i:int;
  if (c < 9) goto B_d;
  c = f_mb(d, c);
  if (c) goto B_c;
  return 0;
  label B_d:
  b = f_bl();
  c = 0;
  b = (b - f_nh(b, 8) + f_nh(20, 8) + f_nh(16, 8) + -65544 & -9) + -3;
  if (select_if(b, e = 0 - (f_nh(16, 8) << 2), e > b) <= d) goto B_b;
  e = f_nh(select_if(16, d + 4, f_nh(16, 8) + -5 > d), 8);
  b = f_nk(a);
  var g:int_ptr = f_kk(b, f = f_ej(b));
  if (f_pi(b)) goto B_l;
  if (f >= e) goto B_k;
  if (g == 0[269237]:int) goto B_j;
  if (g == 0[269236]:int) goto B_i;
  if (f_ei(g)) goto B_e;
  h = f_ej(g);
  f = h + f;
  if (f < e) goto B_e;
  i = f - e;
  if (h < 256) goto B_h;
  f_nc(g);
  goto B_g;
  label B_l:
  f = f_ej(b);
  if (e < 256) goto B_e;
  if (f < e + 4) goto B_m;
  if (f - e < 131073) goto B_f;
  label B_m:
  e = f_dl(1076536,
           b - (g = b.a),
           h = f + g + 16,
           f = f_nh(e + 31, f_hl(1076536)),
           1);
  if (eqz(e)) goto B_e;
  b = e + g;
  b.b = (c = (d = f - g) + -16);
  a = f_al();
  f_kk(b, c)[1]:int = a;
  f_kk(b, d + -12)[1]:int = 0;
  0[269238]:int = (d = 0[269238]:int + f - h);
  0[269245]:int = select_if(c = 0[269245]:int, e, e > c);
  0[269239]:int = select_if(c = 0[269239]:int, d, c > d);
  goto B_a;
  label B_k:
  f = f - e;
  if (f < f_nh(16, 8)) goto B_f;
  g = f_kk(b, e);
  f_ig(b, e);
  f_ig(g, f);
  f_ya(g, f);
  goto B_f;
  label B_j:
  f = 0[269235]:int + f;
  if (f <= e) goto B_e;
  g = f_kk(b, e);
  f_ig(b, e);
  g[1] = (e = f - e) | 1;
  0[269235]:int = e;
  0[269237]:int = g;
  goto B_f;
  label B_i:
  f = 0[269234]:int + f;
  if (f < e) goto B_e;
  g = f - e;
  if (g >= f_nh(16, 8)) goto B_o;
  f_ig(b, f);
  g = 0;
  f = 0;
  goto B_n;
  label B_o:
  f = f_kk(b, e);
  h = f_kk(f, g);
  f_ig(b, e);
  f_gh(f, g);
  f_fi(h);
  label B_n:
  0[269236]:int = f;
  0[269234]:int = g;
  goto B_f;
  label B_h:
  var j:int_ptr = (g + 12)[0]:int;
  if (j == (g = (g + 8)[0]:int)) goto B_p;
  g[3] = j;
  j[2] = g;
  goto B_g;
  label B_p:
  0[269134]:int = 0[269134]:int & -2 << (h >> 3);
  label B_g:
  if (i < f_nh(16, 8)) goto B_q;
  f = f_kk(b, e);
  f_ig(b, e);
  f_ig(f, i);
  f_ya(f, i);
  goto B_f;
  label B_q:
  f_ig(b, f);
  label B_f:
  if (b) goto B_a;
  label B_e:
  e = f_e(d);
  if (eqz(e)) goto B_b;
  d = 
    f_dk(e, a, select_if(d, c = f_ej(b) + select_if(-8, -4, f_pi(b)), c > d));
  f_x(a);
  return d;
  label B_c:
  f_dk(c, a, select_if(d, b, b > d));
  f_x(a);
  label B_b:
  return c;
  label B_a:
  f_pi(b);
  return f_mk(b);
}

function f_ga(a:{ a:int, b:int }, b:int_ptr, c:int) {
  var g:int;
  var e:int_ptr;
  var f:int;
  var h:int;
  var i:int;
  var l:long;
  var k:long;
  var m:long;
  var d:int = g_a - 16;
  g_a = d;
  e = b + 8;
  f = e[0];
  if (f < (g = (b + 4)[0]:int)) goto B_b;
  d[0]:int = 5;
  h = f_ef(b, d);
  a.a = 1;
  a.b = h;
  goto B_a;
  label B_b:
  e[0] = (h = f + 1);
  i = b[0];
  f = (i + f)[0]:ubyte;
  if (f != 48) goto B_n;
  if (h >= g) goto B_o;
  h = (i + h)[0]:ubyte;
  if ((h + -48 & 255) < 10) goto B_k;
  if (h == 46) goto B_l;
  if (h == 69) goto B_m;
  if (h == 101) goto B_m;
  label B_o:
  k = select_if(0L, -9223372036854775808L, c);
  l = i64_extend_i32_u(c);
  goto B_i;
  label B_n:
  if ((f + -49 & 255) < 9) goto B_p;
  d[0]:int = 12;
  h = f_ef(b, d);
  a.a = 1;
  a.b = h;
  goto B_a;
  label B_p:
  l = i64_extend_i32_u(f + -48) & 255L;
  if (h >= g) goto B_h;
  var j:int_ptr = b + 8;
  loop L_q {
    f = (i + h)[0]:ubyte + -48;
    e = f & 255;
    if (e >= 10) goto B_h;
    if (l < 1844674407370955161L) goto B_s;
    if (l != 1844674407370955161L) goto B_r;
    if (e > 5) goto B_r;
    label B_s:
    j[0] = (h = h + 1);
    l = l * 10L + (i64_extend_i32_u(f) & 255L);
    if (g != h) continue L_q;
    goto B_g;
    label B_r:
  }
  f_kb(d, b, c, l);
  if (d[0]:int) goto B_t;
  (a + 16)[0]:double = d[1]:double;
  (a + 8)[0]:long = 0L;
  a.a = 0;
  goto B_a;
  label B_t:
  a.b = d[1]:int;
  a.a = 1;
  goto B_a;
  label B_m:
  l = 0L;
  f_ua(d, b, c, 0L, 0);
  if (eqz(d[0]:int)) goto B_j;
  a.b = d[1]:int;
  a.a = 1;
  goto B_a;
  label B_l:
  l = 0L;
  f_xa(d, b, c, 0L, 0);
  if (eqz(d[0]:int)) goto B_j;
  a.b = d[1]:int;
  a.a = 1;
  goto B_a;
  label B_k:
  d[0]:int = 12;
  h = f_df(b, d);
  a.a = 1;
  a.b = h;
  goto B_a;
  label B_j:
  k = d[1]:long;
  label B_i:
  a.a = 0;
  (a + 16)[0]:long = k;
  (a + 8)[0]:long = l;
  goto B_a;
  label B_h:
  if (h >= g) goto B_g;
  h = (i + h)[0]:ubyte;
  if (h == 46) goto B_e;
  if (h == 69) goto B_f;
  if (h == 101) goto B_f;
  label B_g:
  k = 1L;
  if (eqz(c)) goto B_u;
  m = l;
  goto B_c;
  label B_u:
  k = 0L;
  m = 0L - l;
  if (m >= 0L) goto B_v;
  k = 2L;
  goto B_c;
  label B_v:
  m = i64_reinterpret_f64(f64_convert_i64_u(l)) ^ -9223372036854775808L;
  goto B_c;
  label B_f:
  f_ua(d, b, c, l, 0);
  if (eqz(d[0]:int)) goto B_d;
  a.b = d[1]:int;
  a.a = 1;
  goto B_a;
  label B_e:
  f_xa(d, b, c, l, 0);
  if (eqz(d[0]:int)) goto B_d;
  a.b = d[1]:int;
  a.a = 1;
  goto B_a;
  label B_d:
  m = d[1]:long;
  k = 0L;
  label B_c:
  a.a = 0;
  (a + 16)[0]:long = m;
  (a + 8)[0]:long = k;
  label B_a:
  g_a = d + 16;
}

function f_ha(a:{ a:int, b:int }, b:long_ptr) {
  var n:long;
  var m:long;
  var j:int;
  var i:int;
  var g:int;
  var h:int_ptr;
  var c:int = g_a - 96;
  g_a = c;
  var d:long_ptr = (b + 20)[0]:int;
  var l:long = d[0];
  var e:int = (b + 16)[0]:int;
  f_yg(c + 24, 1, (b + 28)[0]:int);
  if (c[6]:int) goto B_f;
  e = e + d + 1;
  (c + 16)[0]:long = (c + 24 + 20)[0]:long@4;
  (c + 8)[0]:long = (c + 36)[0]:long@4;
  c[0]:long = c[7]:long@4;
  b = d + 8;
  l = (l ^ -1L) & -9187201950435737472L;
  var f:int = c + 56 | 4;
  loop L_g {
    if (l != 0L) goto B_i;
    if (b >= e) goto B_e;
    d = d + -160;
    loop L_k {
      l = b[0] & -9187201950435737472L;
      if (l != -9187201950435737472L) goto B_j;
      d = d + -160;
      b = b + 8;
      if (b >= e) goto B_e;
      continue L_k;
    }
    label B_j:
    b = b + 8;
    m = l ^ -9187201950435737472L;
    n = m + -1L & m;
    goto B_h;
    label B_i:
    if (eqz(d)) goto B_e;
    n = l + -1L & l;
    m = l;
    label B_h:
    l = n;
    if (eqz(d)) goto B_e;
    g = d + (0 - (i32_wrap_i64(ctz(m)) >> 3)) * 20;
    h = select_if(g + -20, 0, d);
    i = (h + 4)[0]:int;
    if (i < 0) goto B_n;
    h = h[0];
    if (i) goto B_p;
    j = 1;
    goto B_o;
    label B_p:
    j = f_wh(i, 1);
    if (eqz(j)) goto B_c;
    label B_o:
    h = f_dk(j, h, i);
    j = c[3]:int;
    if (eqz(j)) goto B_q;
    var k:int = c[4]:int;
    if (eqz(k)) goto B_q;
    f_mi(j, k, 1);
    label B_q:
    c[5]:int = i;
    c[4]:int = i;
    c[3]:int = 0;
    if (eqz(h)) goto B_b;
    i = (g + -4)[0]:int;
    if (i < 0) goto B_n;
    m = c[2]:long;
    j = (g + -12)[0]:int;
    if (i) goto B_m;
    g = 1;
    goto B_l;
    label B_n:
    f_zf();
    unreachable;
    label B_m:
    g = f_wh(i, 1);
    if (eqz(g)) goto B_a;
    label B_l:
    g = f_dk(g, j, i);
    c[21]:long@4 = m;
    c[20]:int = h;
    c[9]:int = i;
    c[8]:int = i;
    c[7]:int = g;
    c[24]:byte = 3;
    f_bb(c + 56, c, c + 80, c + 24);
    i = c[56]:ubyte;
    if (i == 6) continue L_g;
    br_table[L_g, L_g, L_g, B_s, B_r, ..B_t](i)
    label B_t:
    f_oa(f);
    continue L_g;
    label B_s:
    i = c[16]:int;
    if (eqz(i)) continue L_g;
    f_mi(c[15]:int, i, 1);
    continue L_g;
    label B_r:
    f_wd(f);
    i = c[16]:int;
    if (eqz(i)) continue L_g;
    i = i32_wrap_i64(i64_extend_i32_u(i) * 24L);
    if (eqz(i)) continue L_g;
    f_mi(c[15]:int, i, 8);
    continue L_g;
  }
  label B_f:
  a.b = c[7]:int;
  a.a = 1;
  goto B_d;
  label B_e:
  (c + 24 + 16)[0]:long = (c + 16)[0]:long;
  (c + 24 + 8)[0]:long = (c + 8)[0]:long;
  c[3]:long = c[0]:long;
  f_ae(a, c + 24);
  label B_d:
  g_a = c + 96;
  return ;
  label B_c:
  f_mj(i, 1);
  unreachable;
  label B_b:
  f_vi(1050392, 43, 1050532);
  unreachable;
  label B_a:
  f_mj(i, 1);
  unreachable;
}

function f_ia(a:{ a:int, b:int, c:int }, b:int, c:int):int {
  var k:int_ptr;
  var j:int;
  var l:int;
  if (eqz(c)) goto B_b;
  var d:int_ptr = a.b;
  var e:int = a.a;
  var f:int = a.c;
  loop L_c {
    if (eqz(f[0]:ubyte)) goto B_d;
    if (eqz(call_indirect(e, 1072592, 4, d[3]))) goto B_d;
    return 1;
    label B_d:
    var g:int = 0;
    var h:int = c;
    loop L_i {
      var i:{ a:ubyte, b:ubyte, c:ubyte, d:ubyte, e:ubyte, f:ubyte, g:ubyte } = 
        b + g;
      if (h < 8) goto B_n;
      a = (i + 3 & -4) - i;
      if (a) goto B_o;
      j = h + -8;
      a = 0;
      goto B_l;
      label B_o:
      a = select_if(h, a, a > h);
      k = 0;
      loop L_p {
        if ((i + k)[0]:ubyte == 10) goto B_j;
        if (a == (k = k + 1)) goto B_m;
        continue L_p;
      }
      label B_n:
      if (eqz(h)) goto B_h;
      k = 0;
      if (i.a == 10) goto B_j;
      if (h == 1) goto B_h;
      k = 1;
      if (i.b == 10) goto B_j;
      if (h == 2) goto B_h;
      k = 2;
      if (i.c == 10) goto B_j;
      if (h == 3) goto B_h;
      k = 3;
      if (i.d == 10) goto B_j;
      if (h == 4) goto B_h;
      k = 4;
      if (i.e == 10) goto B_j;
      if (h == 5) goto B_h;
      k = 5;
      if (i.f == 10) goto B_j;
      if (h == 6) goto B_h;
      k = 6;
      if (i.g != 10) goto B_h;
      goto B_j;
      label B_m:
      if (a > (j = h + -8)) goto B_k;
      label B_l:
      loop L_r {
        k = i + a;
        l = k[0];
        if (
          (((l ^ -1) & (l ^ 168430090) + -16843009) |
           (((k = (k + 4)[0]:int) ^ -1) & (k ^ 168430090) + -16843009)) &
          -2139062144) goto B_q;
        a = a + 8;
        if (a <= j) continue L_r;
      }
      label B_q:
      if (a <= h) goto B_k;
      f_rj(a, h, a);
      unreachable;
      label B_k:
      if (a == h) goto B_h;
      l = a - h;
      i = i + a;
      k = 0;
      loop L_t {
        if ((i + k)[0]:ubyte == 10) goto B_s;
        if (l + (k = k + 1)) continue L_t;
        goto B_h;
      }
      label B_s:
      k = a + k;
      label B_j:
      a = k + g;
      g = a + 1;
      if (g < a) goto B_u;
      if (c < g) goto B_u;
      if ((b + a)[0]:ubyte != 10) goto B_u;
      f[0]:byte = 1;
      if (c <= g) goto B_g;
      a = g;
      if ((b + g)[0]:byte <= -65) goto B_f;
      goto B_e;
      label B_u:
      h = c - g;
      if (c >= g) continue L_i;
    }
    label B_h:
    f[0]:byte = 0;
    g = c;
    label B_g:
    a = c;
    if (c == g) goto B_e;
    label B_f:
    f_eg(b, c, 0, g, a);
    unreachable;
    label B_e:
    if (eqz(call_indirect(e, b, a, d[3]))) goto B_v;
    return 1;
    label B_v:
    if (c > a) goto B_x;
    if (c == a) goto B_w;
    goto B_a;
    label B_x:
    if ((b + a)[0]:byte <= -65) goto B_a;
    label B_w:
    b = b + a;
    c = c - a;
    if (c) continue L_c;
  }
  label B_b:
  return 0;
  label B_a:
  f_eg(b, c, a, c, a);
  return unreachable;
}

function f_ja(a:int, b:int):int {
  var d:int;
  var e:int;
  var c:int = g_a - 144;
  g_a = c;
  if ((a + 16)[0]:int) goto B_g;
  d = a[6]:int;
  e = f_wh(408, 8);
  if (eqz(e)) goto B_e;
  e[66]:int = 0;
  e[201]:short = 1;
  e[0]:long = b[0]:long;
  e[67]:long@4 = a[0]:long@4;
  (e + 8)[0]:long = (b + 8)[0]:long;
  (e + 16)[0]:long = (b + 16)[0]:long;
  (e + 276)[0]:int = (a + 8)[0]:int;
  d[2]:int = 1;
  d[1]:int = e;
  d[0]:int = 0;
  goto B_f;
  label B_g:
  (c + 72 + 8)[0]:int = (a + 20)[0]:int;
  c[9]:long = a[3]:long@4;
  (c + 88 + 8)[0]:int = (a + 8)[0]:int;
  c[11]:long = a[0]:long@4;
  (c + 104 + 16)[0]:long = (b + 16)[0]:long;
  d = c + 104 + 8;
  d[0]:long = (b + 8)[0]:long;
  c[13]:long = b[0]:long;
  f_f(c + 8, c + 72, c + 88, c + 104);
  e = c[16]:int;
  if (c[24]:ubyte != 6) goto B_h;
  a = a[6]:int;
  a[2]:int = a[2]:int + 1;
  goto B_f;
  label B_h:
  (c + 104 + 32)[0]:long = (c + 8 + 32)[0]:long;
  (c + 104 + 24)[0]:long = (c + 8 + 24)[0]:long;
  (c + 104 + 16)[0]:long = (c + 8 + 16)[0]:long;
  d[0]:long = (c + 8 + 8)[0]:long;
  c[13]:long = c[1]:long;
  b = a[6]:int;
  d = b[1]:int;
  if (eqz(d)) goto B_d;
  var f:int = (c + 60)[0]:int;
  var g:long_ptr = (c + 56)[0]:int;
  var h:int = b[0]:int;
  a = f_wh(456, 8);
  if (eqz(a)) goto B_c;
  a[102]:int = d;
  a[201]:short = 0;
  a[66]:int = 0;
  b[1]:int = a;
  d[200]:short = 0;
  d[66]:int = a;
  b[0]:int = h + 1;
  if (h != g) goto B_b;
  d = a[201]:ushort;
  if (d > 10) goto B_a;
  a[201]:short = (h = d + 1);
  g = a + d * 12;
  (g + 276)[0]:int = (c + 8 + 8)[0]:int;
  (g + 268)[0]:long@4 = c[1]:long;
  d = a + d * 24;
  d[0]:long = (g = c + 120)[0];
  (d + 8)[0]:long = (g + 8)[0]:long;
  (d + 16)[0]:long = (g + 16)[0]:long;
  (a + (h << 2) + 408)[0]:int = f;
  f[200]:short = h;
  f[66]:int = a;
  b[2]:int = b[2]:int + 1;
  label B_f:
  g_a = c + 144;
  return e;
  label B_e:
  f_mj(408, 8);
  unreachable;
  label B_d:
  f_rf(1049152, 43, 1049292);
  unreachable;
  label B_c:
  f_mj(456, 8);
  unreachable;
  label B_b:
  f_rf(1048742, 48, 1048792);
  unreachable;
  label B_a:
  f_rf(1048619, 32, 1048808);
  return unreachable;
}

function f_ka(a:int, b:int, c:int):int {
  var h:int;
  var f:int;
  var j:int;
  var d:int;
  var e:byte_ptr;
  var g:int;
  var k:int_ptr;
  var i:int;
  if (a - b >= c) goto B_d;
  d = b + c;
  e = a + c;
  if (c > 15) goto B_e;
  f = a;
  goto B_b;
  label B_e:
  g = e & -4;
  i = 0 - (h = e & 3);
  if (eqz(h)) goto B_f;
  f = b + c + -1;
  loop L_g {
    e = e + -1;
    e[0] = f[0]:ubyte;
    f = f + -1;
    if (g < e) continue L_g;
  }
  label B_f:
  e = g - (f = (j = c - h) & -4);
  h = 0 - f;
  i = d + i;
  if (eqz(i & 3)) goto B_h;
  if (h > -1) goto B_c;
  f = i << 3;
  c = f & 24;
  k = i & -4;
  b = k + -4;
  d = 0 - f & 24;
  f = k[0];
  loop L_i {
    g = g + -4;
    g[0]:int = f << d | (f = b[0]:int) >> c;
    b = b + -4;
    if (g > e) continue L_i;
    goto B_c;
  }
  label B_h:
  if (h > -1) goto B_c;
  b = j + b + -4;
  loop L_j {
    g = g + -4;
    g[0]:int = b[0]:int;
    b = b + -4;
    if (g > e) continue L_j;
    goto B_c;
  }
  label B_d:
  if (c > 15) goto B_l;
  e = a;
  goto B_k;
  label B_l:
  f = a + (d = 0 - a & 3);
  if (eqz(d)) goto B_m;
  e = a;
  g = b;
  loop L_n {
    e[0] = g[0]:ubyte;
    g = g + 1;
    e = e + 1;
    if (e < f) continue L_n;
  }
  label B_m:
  e = f + (j = (i = c - d) & -4);
  h = b + d;
  if (eqz(h & 3)) goto B_p;
  if (j < 1) goto B_o;
  g = h << 3;
  c = g & 24;
  k = h & -4;
  b = k + 4;
  d = 0 - g & 24;
  g = k[0];
  loop L_q {
    f[0]:int = g >> c | (g = b[0]:int) << d;
    b = b + 4;
    f = f + 4;
    if (f < e) continue L_q;
    goto B_o;
  }
  label B_p:
  if (j < 1) goto B_o;
  b = h;
  loop L_r {
    f[0]:int = b[0]:int;
    b = b + 4;
    f = f + 4;
    if (f < e) continue L_r;
  }
  label B_o:
  c = i & 3;
  b = h + j;
  label B_k:
  if (eqz(c)) goto B_a;
  f = e + c;
  loop L_s {
    e[0] = b[0]:ubyte;
    b = b + 1;
    e = e + 1;
    if (e < f) continue L_s;
    goto B_a;
  }
  label B_c:
  b = j & 3;
  if (eqz(b)) goto B_a;
  d = i + h;
  f = e - b;
  label B_b:
  b = d + -1;
  loop L_t {
    e = e + -1;
    e[0] = b[0]:ubyte;
    b = b + -1;
    if (f < e) continue L_t;
  }
  label B_a:
  return a;
}

function f_la(a:int, b:double, c:int, d:int):int {
  var i:long;
  var g:int;
  var h:int;
  var m:long;
  var k:long;
  var f:int;
  var e:int = g_a - 128;
  g_a = e;
  i = i64_reinterpret_f64(b);
  if (eqz(eqz(i & 9223372036854775807L))) goto B_b;
  f = 4;
  goto B_a;
  label B_b:
  var j:long = i & 4503599627370495L;
  k = select_if(j | 4503599627370496L,
                i << 1L & 9007199254740990L,
                g = i32_wrap_i64(i >> 52L) & 2047);
  var l:long = k & 1L;
  m = i & 9218868437227405312L;
  if (eqz(m)) goto B_d;
  if (m != 9218868437227405312L) goto B_c;
  f = select_if(3, 2, eqz(j));
  goto B_a;
  label B_d:
  g = g + -1075;
  f = i32_wrap_i64(l) ^ 1;
  m = 1L;
  goto B_a;
  label B_c:
  k = 
    select_if(18014398509481984L, k << 1L, h = k == 4503599627370496L);
  m = select_if(2L, 1L, h);
  f = i32_wrap_i64(l) ^ 1;
  g = select_if(-1077, -1076, h) + g;
  label B_a:
  e[60]:short = g;
  e[14]:long = m;
  e[13]:long = 1L;
  e[12]:long = k;
  e[122]:byte = f;
  if (f != 2) goto B_f;
  g = 1072168;
  c = 0;
  goto B_e;
  label B_f:
  if (c) goto B_g;
  g = select_if(1072163, 1072168, i < 0L);
  c = i32_wrap_i64(i >> 63L);
  goto B_e;
  label B_g:
  g = select_if(1072163, 1072164, i < 0L);
  c = 1;
  label B_e:
  f = f + -2;
  br_table[B_l, B_k, B_i, B_j, ..B_l](select_if(f, 3, f < 3) & 255);
  label B_l:
  e[10]:int = 3;
  e[9]:int = 1072172;
  e[16]:short = 2;
  e[21]:int = c;
  e[20]:int = g;
  e[22]:int = e + 32;
  f = 1;
  goto B_h;
  label B_k:
  e[10]:int = 3;
  e[9]:int = 1072169;
  e[16]:short = 2;
  e[21]:int = c;
  e[20]:int = g;
  e[22]:int = e + 32;
  f = 1;
  goto B_h;
  label B_j:
  f_g(e + 32, e + 96, e + 15, 17);
  if (e[8]:int) goto B_n;
  f_a(e + 80, e + 96, e + 15, 17);
  goto B_m;
  label B_n:
  (e + 80 + 8)[0]:int = (e + 32 + 8)[0]:int;
  e[10]:long = e[4]:long;
  label B_m:
  f_nb(e, e[20]:int, e[21]:int, e[44]:ushort, d, e + 32, 4);
  e[21]:int = c;
  e[20]:int = g;
  e[22]:int = e[0]:int;
  f = e[1]:int;
  goto B_h;
  label B_i:
  f = 2;
  e[16]:short = 2;
  if (eqz(d)) goto B_o;
  (e + 48)[0]:int = 1;
  e[22]:short = 0;
  e[10]:int = 2;
  e[9]:int = 1072160;
  e[21]:int = c;
  e[20]:int = g;
  e[22]:int = e + 32;
  goto B_h;
  label B_o:
  f = 1;
  e[10]:int = 1;
  e[9]:int = 1072168;
  e[21]:int = c;
  e[20]:int = g;
  e[22]:int = e + 32;
  label B_h:
  (e + 92)[0]:int = f;
  f = f_wa(a, e + 80);
  g_a = e + 128;
  return f;
}

function f_ma(a:int, b:int, c:int) {
  var d:int;
  var g:int_ptr;
  var i:int;
  var h:int;
  var j:long;
  var k:long;
  if (eqz(c)) goto B_a;
  var e:int = select_if(0, d = c + -7, d > c);
  var f:int = (b + 3 & -4) - b;
  d = 0;
  loop L_f {
    g = (b + d)[0]:ubyte;
    h = (g << 24) >> 24;
    if (h < 0) goto B_i;
    if (f == -1) goto B_h;
    if (f - d & 3) goto B_h;
    if (d >= e) goto B_j;
    loop L_k {
      g = b + d;
      if ((g[0] | (g + 4)[0]:int) & -2139062144) goto B_j;
      d = d + 8;
      if (d < e) continue L_k;
    }
    label B_j:
    if (d >= c) goto B_g;
    loop L_l {
      if ((b + d)[0]:byte < 0) goto B_g;
      if (c != (d = d + 1)) continue L_l;
      goto B_a;
    }
    label B_i:
    j = 1099511627776L;
    k = 4294967296L;
    br_table[B_u, B_t, B_s, ..B_b]((g + 1073280)[0]:ubyte + -2)
    label B_u:
    g = d + 1;
    if (g < c) goto B_n;
    j = 0L;
    goto B_c;
    label B_t:
    j = 0L;
    i = d + 1;
    if (i >= c) goto B_c;
    i = (b + i)[0]:byte;
    br_table[B_r, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_p, B_q, ..B_p](
      g + -224);
    label B_s:
    j = 0L;
    i = d + 1;
    if (i >= c) goto B_c;
    i = (b + i)[0]:byte;
    br_table[B_x, B_y, B_y, B_y, B_w, ..B_y](g + -240)
    label B_y:
    if ((h + 15 & 255) > 2) goto B_d;
    if (i > -1) goto B_d;
    if (i >= -64) goto B_d;
    goto B_v;
    label B_x:
    if ((i + 112 & 255) >= 48) goto B_d;
    goto B_v;
    label B_w:
    if (i > -113) goto B_d;
    label B_v:
    g = d + 2;
    if (g >= c) goto B_c;
    if ((b + g)[0]:byte > -65) goto B_e;
    k = 0L;
    g = d + 3;
    if (g >= c) goto B_b;
    if ((b + g)[0]:byte <= -65) goto B_m;
    j = 3298534883328L;
    k = 4294967296L;
    goto B_b;
    label B_r:
    if ((i & -32) != -96) goto B_d;
    goto B_o;
    label B_q:
    if (i >= -96) goto B_d;
    goto B_o;
    label B_p:
    if ((h + 31 & 255) < 12) goto B_z;
    if ((h & -2) != -18) goto B_d;
    if (i > -1) goto B_d;
    if (i >= -64) goto B_d;
    goto B_o;
    label B_z:
    if (i > -65) goto B_d;
    label B_o:
    k = 0L;
    g = d + 2;
    if (g >= c) goto B_b;
    if ((b + g)[0]:byte > -65) goto B_e;
    goto B_m;
    label B_n:
    j = 1099511627776L;
    k = 4294967296L;
    if ((b + g)[0]:byte > -65) goto B_b;
    label B_m:
    d = g + 1;
    goto B_g;
    label B_h:
    d = d + 1;
    label B_g:
    if (d < c) continue L_f;
    goto B_a;
  }
  label B_e:
  j = 2199023255552L;
  k = 4294967296L;
  goto B_b;
  label B_d:
  j = 1099511627776L;
  k = 4294967296L;
  goto B_b;
  label B_c:
  k = 0L;
  label B_b:
  a[1]:long@4 = (j | i64_extend_i32_u(d)) | k;
  a[0]:int = 1;
  return ;
  label B_a:
  a[1]:int = b;
  (a + 8)[0]:int = c;
  a[0]:int = 0;
}

function f_na(a:int, b:uint_ptr@1, c:int) {
  var d:long_ptr;
  var e:int;
  var g:long_ptr;
  var f:long_ptr;
  var h:long;
  var i:long;
  var j:long;
  var k:long;
  var l:long;
  var m:long;
  a[14]:int = a[14]:int + c;
  d = a[15]:int;
  if (d) goto B_e;
  e = 0;
  goto B_d;
  label B_e:
  e = 8 - d;
  f = select_if(e, c, e < c);
  if (f > 3) goto B_g;
  h = 0L;
  g = 0;
  goto B_f;
  label B_g:
  h = b[0];
  g = 4;
  label B_f:
  if ((g | 1) >= f) goto B_h;
  h = (b + g)[0]:ushort@1 << i64_extend_i32_u(g << 3) | h;
  g = g | 2;
  label B_h:
  if (g >= f) goto B_i;
  h = (b + g)[0]:ubyte << i64_extend_i32_u(g << 3) | h;
  label B_i:
  a[6]:long = (h = a[6]:long | h << i64_extend_i32_u(d << 3 & 56));
  if (e > c) goto B_c;
  g = a + 32;
  g[0] =
    (m = (j = (d = a + 24)[0] + (i = (f = a + 40)[0] ^ h)) +
         (l = (k = g[0]) << 13L ^ (k = k + a[2]:long))) ^
    l << 17L;
  d[0] = m << 32L;
  f[0] = (i = j ^ i << 16L) << 21L ^ (i = i + (k << 32L));
  a[2]:long = i ^ h;
  label B_d:
  c = c - e;
  g = c & 7;
  if (e >= (c = c & -8)) goto B_j;
  i = (a + 24)[0]:long;
  h = (a + 32)[0]:long;
  j = (a + 40)[0]:long;
  k = a[2]:long;
  loop L_k {
    l = (b + e)[0]:long@1;
    j = l ^ j;
    i = j + i;
    m = i + (h = (k = k + h) ^ h << 13L);
    h = m ^ h << 17L;
    i = j << 16L ^ i;
    j = i << 21L ^ (k = i + (k << 32L));
    i = m << 32L;
    k = k ^ l;
    e = e + 8;
    if (e < c) continue L_k;
  }
  a[4]:long = h;
  a[2]:long = k;
  a[5]:long = j;
  a[3]:long = i;
  label B_j:
  if (g > 3) goto B_b;
  h = 0L;
  c = 0;
  goto B_a;
  label B_c:
  a[15]:int = d + c;
  return ;
  label B_b:
  h = (b + e)[0]:uint@1;
  c = 4;
  label B_a:
  if ((c | 1) >= g) goto B_l;
  h = (b + c + e)[0]:ushort@1 << i64_extend_i32_u(c << 3) | h;
  c = c | 2;
  label B_l:
  if (c >= g) goto B_m;
  h = (b + c + e)[0]:ubyte << i64_extend_i32_u(c << 3) | h;
  label B_m:
  a[6]:long = h;
  a[15]:int = g;
}

function f_oa(a:int) {
  var f:int;
  var b:int = g_a - 64;
  g_a = b;
  var c:int_ptr = a[1]:int;
  if (eqz(c)) goto B_a;
  var d:int = a[2]:int;
  a = a[0]:int;
  (b + 8 + 24)[0]:int = c;
  (b + 28)[0]:int = a;
  (b + 8 + 8)[0]:int = c;
  b[10]:int = d;
  b[6]:int = 0;
  b[3]:int = a;
  b[2]:int = 0;
  if (d) goto B_e;
  b[2]:int = 2;
  goto B_d;
  label B_e:
  var e:int = b + 8 | 4;
  f = 0;
  loop L_g {
    b[10]:int = d + -1;
    br_table[B_i, B_h, B_f, ..B_h](f)
    label B_i:
    c = b[4]:int;
    a = b[3]:int;
    if (eqz(a)) goto B_j;
    f = a + -1;
    d = a & 7;
    if (eqz(d)) goto B_k;
    loop L_l {
      a = a + -1;
      c = c[102];
      d = d + -1;
      if (d) continue L_l;
    }
    label B_k:
    if (f < 7) goto B_j;
    loop L_m {
      c = 
        (((((((c[102])[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
      a = a + -8;
      if (a) continue L_m;
    }
    label B_j:
    b[5]:int = 0;
    b[4]:int = c;
    b[1]:long = 1L;
    label B_h:
    f_ic(b + 48, e);
    a = b[13]:int;
    if (eqz(a)) goto B_a;
    d = a + (c = b[14]:int) * 12;
    f = (d + 272)[0]:int;
    if (eqz(f)) goto B_n;
    f_mi((d + 268)[0]:int, f, 1);
    label B_n:
    a = a + c * 24;
    br_table[B_o, B_o, B_o, B_q, B_p, ..B_r](a[0]:ubyte);
    label B_r:
    f_oa(a + 4);
    goto B_o;
    label B_q:
    c = (a + 8)[0]:int;
    if (eqz(c)) goto B_o;
    f_mi((a + 4)[0]:int, c, 1);
    goto B_o;
    label B_p:
    c = a + 4;
    f_wd(c);
    a = (a + 8)[0]:int;
    if (eqz(a)) goto B_o;
    a = i32_wrap_i64(i64_extend_i32_u(a) * 24L);
    if (eqz(a)) goto B_o;
    f_mi(c[0], a, 8);
    label B_o:
    f = b[2]:int;
    d = b[10]:int;
    if (d) continue L_g;
  }
  b[2]:int = 2;
  c = b[4]:int;
  a = b[3]:int;
  br_table[B_d, B_c, B_a, ..B_c](f);
  label B_f:
  f_rf(1050238, 43, 1050376);
  unreachable;
  label B_d:
  if (a) goto B_s;
  a = 0;
  goto B_b;
  label B_s:
  f = a + -1;
  d = a & 7;
  if (eqz(d)) goto B_t;
  loop L_u {
    a = a + -1;
    c = c[102];
    d = d + -1;
    if (d) continue L_u;
  }
  label B_t:
  if (f < 7) goto B_v;
  loop L_w {
    c = 
      (((((((c[102])[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
    a = a + -8;
    if (a) continue L_w;
  }
  label B_v:
  a = 0;
  label B_c:
  if (eqz(c)) goto B_a;
  label B_b:
  loop L_x {
    d = c[66];
    f = select_if(456, 408, a);
    if (eqz(f)) goto B_y;
    f_mi(c, f, 8);
    label B_y:
    a = a + 1;
    c = d;
    if (d) continue L_x;
  }
  label B_a:
  g_a = b + 64;
}

function f_pa(a:{ a:int, b:int }, b:int, c:int, d:int, e:long_ptr) {
  var o:long;
  var l:int;
  var i:int;
  var k:int;
  var h:long_ptr;
  var j:int_ptr@1;
  var f:int = g_a - 80;
  g_a = f;
  f_ma(f, b, c);
  if (f[0]:int) goto B_e;
  f_fb(f, b, c);
  if (f[0]:int) goto B_d;
  var g:int = (f + 36)[0]:int;
  h = (f + 28)[0]:int;
  i = (f + 24)[0]:int;
  f_ma(f, d, e);
  if (f[0]:int) goto B_f;
  j = f_wh(4, 1);
  if (eqz(j)) goto B_c;
  j[0] = 1633771873;
  if (eqz(e)) goto B_g;
  f_mi(d, e, 1);
  label B_g:
  if (eqz(i)) goto B_a;
  if (g) goto B_h;
  k = i + 1;
  goto B_b;
  label B_h:
  e = h + 8;
  g = h + (k = i + 1);
  var n:long = (h[0] ^ -1L) & -9187201950435737472L;
  d = h;
  loop L_i {
    if (n == 0L) goto B_k;
    o = n;
    goto B_j;
    label B_k:
    loop L_l {
      if (e >= g) goto B_b;
      d = d + -160;
      o = e[0];
      l = e + 8;
      e = l;
      o = o & -9187201950435737472L;
      if (o == -9187201950435737472L) continue L_l;
    }
    o = o ^ -9187201950435737472L;
    e = l;
    label B_j:
    n = o + -1L & o;
    l = d + (0 - (i32_wrap_i64(ctz(o)) >> 3)) * 20;
    var m:int = (l + -8)[0]:int;
    if (eqz(m)) continue L_i;
    f_mi((l + -12)[0]:int, m, 1);
    continue L_i;
  }
  label B_f:
  f[15]:long@4 = f[1]:long@4;
  f[14]:int = e;
  f[13]:int = e;
  f[12]:int = d;
  (f + 20)[0]:int = 1;
  f[1]:long@4 = 1L;
  f[0]:int = 1049932;
  f[19]:int = 11;
  f[4]:int = f + 72;
  f[18]:int = f + 48;
  f_ag(f, 1050028);
  unreachable;
  label B_e:
  f[15]:long@4 = f[1]:long@4;
  f[14]:int = c;
  f[13]:int = c;
  f[12]:int = b;
  (f + 20)[0]:int = 1;
  f[1]:long@4 = 1L;
  f[0]:int = 1049932;
  f[19]:int = 11;
  f[4]:int = f + 72;
  f[18]:int = f + 48;
  f_ag(f, 1049940);
  unreachable;
  label B_d:
  f[11]:int = f[1]:int;
  (f + 68)[0]:int = 1;
  f[13]:long@4 = 1L;
  f[12]:int = 1050004;
  f[19]:int = 10;
  f[16]:int = f + 72;
  f[18]:int = f + 44;
  f_ag(f + 48, 1050012);
  unreachable;
  label B_c:
  f_mj(4, 1);
  unreachable;
  label B_b:
  d = i + (e = i32_wrap_i64(i64_extend_i32_u(k) * 20L) + 7 & -8) + 9;
  if (eqz(d)) goto B_a;
  f_mi(h - e, d, 8);
  label B_a:
  if (eqz(c)) goto B_m;
  f_mi(b, c, 1);
  label B_m:
  a.b = 4;
  a.a = j;
  g_a = f + 80;
}

function f_qa(a:{ a:int, b:int, c:int, d:int, e:byte }, b:int_ptr):int {
  var e:int;
  var f:int;
  var i:long;
  var h:int;
  var c:int = g_a - 16;
  g_a = c;
  var d:int = b[0];
  b = b[2];
  (a + 16)[0]:byte = 0;
  a.b = (e = a.b) + 1;
  f_jc(c, f = a.a, 1054416, 1);
  if (c[0]:ubyte != 4) goto B_e;
  var g:int = d + b * 24;
  if (b) goto B_i;
  a.b = e;
  f_jc(c, f, 1054417, 1);
  if (c[0]:ubyte == 4) goto B_h;
  c[1]:long = c[0]:long;
  b = f_cg(c + 8);
  goto B_a;
  label B_i:
  b = 1;
  if (d != g) goto B_g;
  a.b = e;
  goto B_d;
  label B_h:
  b = 0;
  if (d == g) goto B_f;
  label B_g:
  loop L_j {
    if ((b & 255) == 1) goto B_o;
    f_jc(c + 8, f = a.a, 1054425, 2);
    if (c[8]:ubyte == 4) goto B_n;
    goto B_m;
    label B_o:
    f_jc(c + 8, f = a.a, 1054424, 1);
    if (c[8]:ubyte != 4) goto B_m;
    label B_n:
    b = a.b;
    if (eqz(b)) goto B_k;
    e = a.d;
    h = a.c;
    loop L_q {
      f_jc(c + 8, f, h, e);
      if (c[8]:ubyte != 4) goto B_p;
      b = b + -1;
      if (b) continue L_q;
      goto B_k;
    }
    label B_p:
    i = c[1]:long;
    if ((i32_wrap_i64(i) & 255) != 4) goto B_l;
    goto B_k;
    label B_m:
    i = c[1]:long;
    if ((i32_wrap_i64(i) & 255) == 4) goto B_k;
    label B_l:
    c[1]:long = i;
    b = f_cg(c + 8);
    goto B_a;
    label B_k:
    b = f_i(d, a);
    if (b) goto B_a;
    a.e = 1;
    b = 2;
    d = d + 24;
    if (d != g) continue L_j;
  }
  f = a.a;
  a.b = (b = a.b + -1);
  f_jc(c + 8, f, 1054424, 1);
  if (c[8]:ubyte != 4) goto B_r;
  if (eqz(b)) goto B_d;
  e = a.d;
  h = a.c;
  loop L_t {
    f_jc(c + 8, f, h, e);
    if (c[8]:ubyte != 4) goto B_s;
    b = b + -1;
    if (eqz(b)) goto B_d;
    continue L_t;
  }
  label B_s:
  i = c[1]:long;
  if ((i32_wrap_i64(i) & 255) == 4) goto B_d;
  goto B_b;
  label B_r:
  c[0]:long = (i = c[1]:long);
  f = i32_wrap_i64(i);
  goto B_c;
  label B_f:
  b = 0;
  goto B_a;
  label B_e:
  c[1]:long = c[0]:long;
  b = f_cg(c + 8);
  goto B_a;
  label B_d:
  f_jc(c, f, 1054417, 1);
  f = c[0]:ubyte;
  label B_c:
  b = 0;
  if ((f & 255) == 4) goto B_a;
  i = c[0]:long;
  label B_b:
  c[1]:long = i;
  b = f_cg(c + 8);
  label B_a:
  g_a = c + 16;
  return b;
}

function f_ra(a:int_ptr, b:int):int {
  var e:byte_ptr;
  var k:int;
  var d:int;
  var c:int_ptr = g_a - 16;
  g_a = c;
  d = b[1]:int;
  if (eqz(d)) goto B_b;
  e = 1;
  if (
    call_indirect(a[6], b[0]:int, d, ((a + 28)[0]:int)[3]:int)) goto B_a;
  label B_b:
  e = (b + 12)[0]:int;
  if (e) goto B_c;
  e = 0;
  goto B_a;
  label B_c:
  var f:{ a:ushort, b:ushort, c:int, d:int } = b[2]:int;
  var g:int = f + e * 12;
  var h:int_ptr = (a + 28)[0]:int;
  var i:int = a[6];
  var j:byte_ptr = c + 8 + 4;
  loop L_d {
    br_table[B_h, B_f, B_g, ..B_h](f.a)
    label B_h:
    b = f.c;
    if (b < 65) goto B_j;
    a = h[3];
    loop L_k {
      if (eqz(call_indirect(i, 1072860, 64, a))) goto B_l;
      e = 1;
      goto B_a;
      label B_l:
      b = b + -64;
      if (b > 64) continue L_k;
      goto B_i;
    }
    label B_j:
    if (eqz(b)) goto B_e;
    label B_i:
    if (b > 63) goto B_n;
    if ((b + 1072860)[0]:byte <= -65) goto B_m;
    label B_n:
    if (eqz(call_indirect(i, 1072860, b, h[3]))) goto B_e;
    e = 1;
    goto B_a;
    label B_m:
    f_eg(1072860, 64, 0, b, b);
    unreachable;
    label B_g:
    if (eqz(call_indirect(i, f.c, f.d, h[3]))) goto B_e;
    e = 1;
    goto B_a;
    label B_f:
    b = f.b;
    j[0] = 0;
    c[2] = 0;
    a = 1;
    br_table[B_q, B_s, B_r, ..B_q](f.a)
    label B_s:
    a = f.b;
    if (a < 1000) goto B_t;
    k = select_if(4, 5, a < 10000);
    goto B_p;
    label B_t:
    k = 1;
    if (a < 10) goto B_p;
    k = select_if(2, 3, a < 100);
    goto B_p;
    label B_r:
    a = 2;
    label B_q:
    k = f[a]:int;
    if (k >= 6) goto B_u;
    if (k) goto B_p;
    k = 0;
    goto B_o;
    label B_u:
    f_sj(k, 5, b);
    unreachable;
    label B_p:
    e = c + 8 + k;
    if (k & 1) goto B_w;
    a = b;
    goto B_v;
    label B_w:
    e = e + -1;
    e[0] = b - (a = (b & 65535) / 10) * 10 | 48;
    label B_v:
    if (k == 1) goto B_o;
    b = e + -2;
    loop L_x {
      b[0]:byte = (d = (e = a & 65535) / 10) % 10 | 48;
      (b + 1)[0]:byte = a - d * 10 | 48;
      a = e / 100;
      e = b == c + 8;
      b = b + -2;
      if (eqz(e)) continue L_x;
    }
    label B_o:
    if (eqz(call_indirect(i, c + 8, k, h[3]))) goto B_e;
    e = 1;
    goto B_a;
    label B_e:
    if (g != (f = f + 12)) continue L_d;
  }
  e = 0;
  label B_a:
  g_a = c + 16;
  return e;
}

function f_sa(a:int, b:int_ptr, c:int, d:int, e:int) {
  var g:int;
  var n:ubyte_ptr;
  var o:int;
  var q:int;
  var p:int;
  var r:long;
  var f:int = g_a - 16;
  g_a = f;
  f_jc(f, g = b[0], 1054385, 1);
  if (f[0]:ubyte != 4) goto B_b;
  var h:int = d + -1;
  var i:int = e ^ -1;
  var j:int = d + e;
  var k:int = 0;
  var l:int = d;
  var m:int = 0;
  loop L_g {
    b = 0;
    loop L_l {
      n = l + b;
      if (n != j) goto B_m;
      if (m == e) goto B_f;
      if (eqz(m)) goto B_k;
      if (m >= e) goto B_n;
      if ((d + m)[0]:byte > -65) goto B_k;
      label B_n:
      f_eg(d, e, m, e, 1054448);
      unreachable;
      label B_m:
      b = b + 1;
      o = n[0];
      n = (o + 1054464)[0]:ubyte;
      if (eqz(n)) continue L_l;
    }
    p = k + b;
    q = p + -1;
    if (q <= m) goto B_i;
    if (eqz(m)) goto B_o;
    if (m < e) goto B_p;
    if (m == e) goto B_o;
    goto B_j;
    label B_p:
    if ((d + m)[0]:byte < -64) goto B_j;
    label B_o:
    if (q < e) goto B_r;
    q = e;
    if (i + k + b) goto B_j;
    goto B_q;
    label B_r:
    if ((h + k + b)[0]:byte <= -65) goto B_j;
    label B_q:
    f_jc(f, g, d + m, q - m);
    if (f[0]:ubyte == 4) goto B_i;
    goto B_h;
    label B_k:
    f_jc(f, g, d + m, e - m);
    if (f[0]:ubyte == 4) goto B_f;
    goto B_h;
    label B_j:
    f_eg(d, e, m, k + b + -1, 1054432);
    unreachable;
    label B_i:
    br_table[B_u, B_c, B_c, B_c, B_c, B_c, B_aa, B_c, B_c, B_c, B_z, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_y, B_c, B_c, B_c, B_x, B_c, B_w, B_v, ..B_ba](
      n + -92)
    label B_ba:
    if (n != 34) goto B_c;
    n = 1054398;
    goto B_t;
    label B_aa:
    n = 1054394;
    goto B_t;
    label B_z:
    n = 1054392;
    goto B_t;
    label B_y:
    n = 1054390;
    goto B_t;
    label B_x:
    n = 1054388;
    goto B_t;
    label B_w:
    n = 1054386;
    goto B_t;
    label B_v:
    f[10]:int@1 = 808482140;
    f[15]:byte = ((o & 15) + 1054400)[0]:ubyte;
    f[14]:byte = ((o >> 4) + 1054400)[0]:ubyte;
    f_jc(f, g, f + 10, 6);
    goto B_s;
    label B_u:
    n = 1054396;
    label B_t:
    f_jc(f, g, n, 2);
    label B_s:
    if (f[0]:ubyte != 4) goto B_h;
    l = l + b;
    m = q + 1;
    k = p;
    continue L_g;
    label B_h:
  }
  r = f[0]:long;
  if ((i32_wrap_i64(r) & 255) != 4) goto B_e;
  label B_f:
  f_jc(f, g, 1054385, 1);
  if (f[0]:ubyte != 4) goto B_d;
  a[0]:byte = 4;
  goto B_a;
  label B_e:
  a[0]:long@4 = r;
  goto B_a;
  label B_d:
  a[0]:long@4 = f[0]:long;
  goto B_a;
  label B_c:
  f_rf(1054225, 40, 1054356);
  unreachable;
  label B_b:
  a[0]:long@4 = f[0]:long;
  label B_a:
  g_a = f + 16;
}

function f_ta(a:int, b:int_ptr, c:int, d:int, e:int) {
  var g:int;
  var n:ubyte_ptr;
  var o:int;
  var q:int;
  var p:int;
  var r:long;
  var f:int = g_a - 16;
  g_a = f;
  f_jc(f, g = b[0], 1054385, 1);
  if (f[0]:ubyte != 4) goto B_b;
  var h:int = d + -1;
  var i:int = e ^ -1;
  var j:int = d + e;
  var k:int = 0;
  var l:int = d;
  var m:int = 0;
  loop L_g {
    b = 0;
    loop L_l {
      n = l + b;
      if (n != j) goto B_m;
      if (m == e) goto B_f;
      if (eqz(m)) goto B_k;
      if (m >= e) goto B_n;
      if ((d + m)[0]:byte > -65) goto B_k;
      label B_n:
      f_eg(d, e, m, e, 1054448);
      unreachable;
      label B_m:
      b = b + 1;
      o = n[0];
      n = (o + 1054464)[0]:ubyte;
      if (eqz(n)) continue L_l;
    }
    p = k + b;
    q = p + -1;
    if (q <= m) goto B_i;
    if (eqz(m)) goto B_o;
    if (m < e) goto B_p;
    if (m == e) goto B_o;
    goto B_j;
    label B_p:
    if ((d + m)[0]:byte < -64) goto B_j;
    label B_o:
    if (q < e) goto B_r;
    q = e;
    if (i + k + b) goto B_j;
    goto B_q;
    label B_r:
    if ((h + k + b)[0]:byte <= -65) goto B_j;
    label B_q:
    f_jc(f, g, d + m, q - m);
    if (f[0]:ubyte == 4) goto B_i;
    goto B_h;
    label B_k:
    f_jc(f, g, d + m, e - m);
    if (f[0]:ubyte == 4) goto B_f;
    goto B_h;
    label B_j:
    f_eg(d, e, m, k + b + -1, 1054432);
    unreachable;
    label B_i:
    br_table[B_u, B_c, B_c, B_c, B_c, B_c, B_aa, B_c, B_c, B_c, B_z, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_y, B_c, B_c, B_c, B_x, B_c, B_w, B_v, ..B_ba](
      n + -92)
    label B_ba:
    if (n != 34) goto B_c;
    n = 1054398;
    goto B_t;
    label B_aa:
    n = 1054394;
    goto B_t;
    label B_z:
    n = 1054392;
    goto B_t;
    label B_y:
    n = 1054390;
    goto B_t;
    label B_x:
    n = 1054388;
    goto B_t;
    label B_w:
    n = 1054386;
    goto B_t;
    label B_v:
    f[10]:int@1 = 808482140;
    f[15]:byte = ((o & 15) + 1054400)[0]:ubyte;
    f[14]:byte = ((o >> 4) + 1054400)[0]:ubyte;
    f_jc(f, g, f + 10, 6);
    goto B_s;
    label B_u:
    n = 1054396;
    label B_t:
    f_jc(f, g, n, 2);
    label B_s:
    if (f[0]:ubyte != 4) goto B_h;
    l = l + b;
    m = q + 1;
    k = p;
    continue L_g;
    label B_h:
  }
  r = f[0]:long;
  if ((i32_wrap_i64(r) & 255) != 4) goto B_e;
  label B_f:
  f_jc(f, g, 1054385, 1);
  if (f[0]:ubyte != 4) goto B_d;
  a[0]:byte = 4;
  goto B_a;
  label B_e:
  a[0]:long@4 = r;
  goto B_a;
  label B_d:
  a[0]:long@4 = f[0]:long;
  goto B_a;
  label B_c:
  f_rf(1054225, 40, 1054356);
  unreachable;
  label B_b:
  a[0]:long@4 = f[0]:long;
  label B_a:
  g_a = f + 16;
}

function f_ua(a:{ a:int, b:int }, b:int_ptr, c:int, d:long, e:int) {
  var i:int;
  var k:int;
  var f:int_ptr = g_a - 16;
  g_a = f;
  var g:int = 1;
  var h:int_ptr = b + 8;
  h[0] = (i = (h = h[0]) + 1);
  var j:int = b[0];
  if (i >= (k = (b + 4)[0]:int)) goto B_a;
  g = 1;
  br_table[B_b, B_a, B_c, ..B_a]((j + i)[0]:ubyte + -43)
  label B_c:
  g = 0;
  label B_b:
  (b + 8)[0]:int = (i = h + 2);
  label B_a:
  if (i < k) goto B_e;
  f[0] = 5;
  h = f_ef(b, f);
  a.a = 1;
  a.b = h;
  goto B_d;
  label B_e:
  (b + 8)[0]:int = (h = i + 1);
  i = (j + i)[0]:ubyte + -48 & 255;
  if (i < 10) goto B_f;
  f[0] = 12;
  h = f_ef(b, f);
  a.a = 1;
  a.b = h;
  goto B_d;
  label B_f:
  if (h >= k) goto B_g;
  var l:int_ptr = b + 8;
  loop L_h {
    var m:int = (j + h)[0]:ubyte + -48 & 255;
    if (m >= 10) goto B_g;
    l[0] = (h = h + 1);
    if (i < 214748364) goto B_j;
    if (i != 214748364) goto B_i;
    if (m > 7) goto B_i;
    label B_j:
    i = i * 10 + m;
    if (k != h) continue L_h;
    goto B_g;
    label B_i:
  }
  f_qd(a, b, c, eqz(d), g);
  goto B_d;
  label B_g:
  if (g) goto B_l;
  h = e - i;
  h = select_if(h >> 31 ^ -2147483648, h, i > 0 ^ h < e);
  goto B_k;
  label B_l:
  h = e + i;
  h = select_if(h >> 31 ^ -2147483648, h, i < 0 ^ h < e);
  label B_k:
  var n:double = f64_convert_i64_u(d);
  i = h + (i = h >> 31) ^ i;
  if (i < 309) goto B_q;
  loop L_r {
    if (n == 0.0) goto B_n;
    if (h > -1) goto B_p;
    n = 
      n /
      100000000000000001097906362944045541740492309677311846336810682903157585404911491537163328978494688899061249669721172515611590283743140088328307009198146046031271664502933027185697489699588559043338384466165001178426897626212945177628091195786707458122783970171784415105291802893207873272974885715430223118336.0;
    h = h + 308;
    i = h + (i = h >> 31) ^ i;
    if (i >= 309) continue L_r;
  }
  label B_q:
  var o:double = ((i << 3) + 1050552)[0]:double;
  if (h > -1) goto B_s;
  n = n / o;
  goto B_n;
  label B_s:
  n = n * o;
  if (
    f64_reinterpret_i64(i64_reinterpret_f64(n) & 9223372036854775807L) !=
    inf) goto B_n;
  f[0] = 13;
  a.b = f_ef(b, f);
  goto B_o;
  label B_p:
  f[0] = 13;
  a.b = f_ef(b, f);
  label B_o:
  h = 1;
  goto B_m;
  label B_n:
  (a + 8)[0]:double = select_if(n, -(n), c);
  h = 0;
  label B_m:
  a.a = h;
  label B_d:
  g_a = f + 16;
}

function f_va(a:{ a:int, b:int }, b:{ a:int, b:int }, c:int_ptr):int {
  var f:int;
  var g:int;
  var h:int;
  var d:int = g_a - 48;
  g_a = d;
  (d + 36)[0]:int = b;
  d[40]:byte = 3;
  d[1]:long = 137438953472L;
  d[8]:int = a;
  var e:int = 0;
  d[6]:int = 0;
  d[4]:int = 0;
  f = c[2];
  if (f) goto B_d;
  g = (c + 20)[0]:int;
  if (eqz(g)) goto B_c;
  b = c[0];
  a = c[4];
  e = (g + -1 & 536870911) + 1;
  g = e;
  loop L_e {
    h = (b + 4)[0]:int;
    if (eqz(h)) goto B_f;
    if (call_indirect(d[8]:int, b.a, h, (d[9]:int)[3]:int)) goto B_b;
    label B_f:
    if (call_indirect(a.a, d + 8, (a + 4)[0]:int)) goto B_b;
    a = a + 8;
    b = b + 8;
    g = g + -1;
    if (g) continue L_e;
    goto B_c;
  }
  label B_d:
  a = (c + 12)[0]:int;
  if (eqz(a)) goto B_c;
  var i:int = a << 5;
  e = (a + -1 & 134217727) + 1;
  b = c[0];
  g = 0;
  loop L_g {
    a = (b + 4)[0]:int;
    if (eqz(a)) goto B_h;
    if (call_indirect(d[8]:int, b.a, a, (d[9]:int)[3]:int)) goto B_b;
    label B_h:
    d[40]:byte = ((a = f + g) + 28)[0]:ubyte;
    d[1]:long = (a + 4)[0]:long@4 << 32L;
    var j:{ a:int, b:int } = (a + 24)[0]:int;
    var k:int = c[4];
    var l:int = 0;
    h = 0;
    br_table[B_j, B_k, B_i, ..B_j]((a + 20)[0]:int)
    label B_k:
    var m:{ a:int, b:int } = j << 3;
    h = 0;
    m = k + m;
    if (m.b != 72) goto B_i;
    j = m.a[0]:int;
    label B_j:
    h = 1;
    label B_i:
    d[5]:int = j;
    d[4]:int = h;
    h = (a + 16)[0]:int;
    br_table[B_m, B_n, B_l, ..B_m]((a + 12)[0]:int)
    label B_n:
    j = h << 3;
    j = k + j;
    if (j.b != 72) goto B_l;
    h = j.a[0]:int;
    label B_m:
    l = 1;
    label B_l:
    d[7]:int = h;
    d[6]:int = l;
    a = k + (a.a << 3);
    if (call_indirect(a.a, d + 8, a.b)) goto B_b;
    b = b + 8;
    if (i != (g = g + 32)) continue L_g;
  }
  label B_c:
  a = 0;
  b = e < c[1];
  if (eqz(b)) goto B_a;
  if (
    eqz(call_indirect(d[8]:int,
                      (b = select_if(c[0] + (e << 3), 0, b)).a,
                      b.b,
                      (d[9]:int)[3]:int))) goto B_a;
  label B_b:
  a = 1;
  label B_a:
  g_a = d + 48;
  return a;
}

function f_wa(a:int, b:int_ptr):int {
  var e:int;
  var f:ushort_ptr;
  var g:int;
  var d:int;
  var l:int;
  var k:int;
  var j:int;
  var h:int;
  var i:int;
  var c:{ a:int, b:int, c:int } = g_a - 16;
  g_a = c;
  if (a[2]:int != 1) goto B_e;
  d = (a + 12)[0]:int;
  (c + 12)[0]:int = (e = (b + 12)[0]:int);
  c.c = (f = (b + 8)[0]:int);
  c.b = (g = (b + 4)[0]:int);
  c.a = (b = b[0]);
  h = a[32]:ubyte;
  i = a[1]:int;
  if (a[0]:ubyte & 8) goto B_d;
  j = i;
  b = g;
  k = h;
  goto B_c;
  label B_e:
  f = f_ra(a, b);
  goto B_a;
  label B_d:
  if (call_indirect(a[6]:int, b, g, ((a + 28)[0]:int)[3]:int)) goto B_b;
  k = 1;
  a[32]:byte = 1;
  j = 48;
  a[1]:int = 48;
  b = 0;
  c.b = 0;
  c.a = 1072168;
  d = select_if(0, g = d - g, g > d);
  label B_c:
  if (eqz(e)) goto B_f;
  e = e * 12;
  loop L_g {
    br_table[B_k, B_i, B_j, ..B_k](f[0])
    label B_k:
    g = (f + 4)[0]:int;
    goto B_h;
    label B_j:
    g = (f + 8)[0]:int;
    goto B_h;
    label B_i:
    l = (f + 2)[0]:ushort;
    if (l < 1000) goto B_l;
    g = select_if(4, 5, l < 10000);
    goto B_h;
    label B_l:
    g = 1;
    if (l < 10) goto B_h;
    g = select_if(2, 3, l < 100);
    label B_h:
    f = f + 12;
    b = g + b;
    e = e + -12;
    if (e) continue L_g;
  }
  label B_f:
  if (d <= b) goto B_o;
  f = 0;
  b = d - b;
  e = b;
  br_table[B_p, B_r, B_q, B_r, ..B_p](k & 3)
  label B_r:
  e = 0;
  f = b;
  goto B_p;
  label B_q:
  f = b >> 1;
  e = b + 1 >> 1;
  label B_p:
  f = f + 1;
  b = (a + 28)[0]:int;
  g = a[6]:int;
  loop L_s {
    f = f + -1;
    if (eqz(f)) goto B_n;
    if (eqz(call_indirect(g, j, b[4]))) continue L_s;
    goto B_b;
  }
  label B_o:
  f = f_ra(a, c);
  goto B_m;
  label B_n:
  if (f_ra(a, c)) goto B_b;
  f = 0;
  loop L_t {
    if (e != f) goto B_u;
    f = e < e;
    goto B_m;
    label B_u:
    f = f + 1;
    if (eqz(call_indirect(g, j, b[4]))) continue L_t;
  }
  f = f + -1 < e;
  label B_m:
  a[32]:byte = h;
  a[1]:int = i;
  goto B_a;
  label B_b:
  f = 1;
  label B_a:
  g_a = c + 16;
  return f;
}

function f_xa(a:{ a:int, b:int }, b:int_ptr, c:int, d:long, e:int) {
  var g:int;
  var h:int;
  var i:int;
  var o:int;
  var n:int;
  var m:int;
  var p:double;
  var f:int_ptr = g_a - 16;
  g_a = f;
  b[2] = (h = (g = b[2]) + 1);
  if (h >= (i = (b + 4)[0]:int)) goto B_i;
  var j:int = b[0] + h;
  var k:int = g - i + 1;
  var l:int = g + e - i + 1;
  h = 0;
  loop L_j {
    m = (j + h)[0]:ubyte;
    n = m + -48;
    o = n & 255;
    if (o < 10) goto B_k;
    if (e != h) goto B_f;
    h = g + h + 1;
    goto B_h;
    label B_k:
    if (d < 1844674407370955161L) goto B_l;
    if (d != 1844674407370955161L) goto B_g;
    if (o > 5) goto B_g;
    label B_l:
    b[2] = g + h + 2;
    d = d * 10L + (i64_extend_i32_u(n) & 255L);
    if (k + (h = h + 1)) continue L_j;
  }
  h = i;
  e = l;
  label B_i:
  if (e) goto B_e;
  label B_h:
  if (h < i) goto B_c;
  f[0] = 5;
  h = f_df(b, f);
  a.a = 1;
  a.b = h;
  goto B_a;
  label B_g:
  f_ub(a, b, c, d, e - h);
  goto B_a;
  label B_f:
  e = e - h;
  if ((m | 32) == 101) goto B_d;
  label B_e:
  p = f64_convert_i64_u(d);
  h = e + (h = e >> 31) ^ h;
  if (h < 309) goto B_n;
  loop L_o {
    if (p == 0.0) goto B_b;
    if (e > -1) goto B_m;
    p = 
      p /
      100000000000000001097906362944045541740492309677311846336810682903157585404911491537163328978494688899061249669721172515611590283743140088328307009198146046031271664502933027185697489699588559043338384466165001178426897626212945177628091195786707458122783970171784415105291802893207873272974885715430223118336.0;
    e = e + 308;
    h = e + (h = e >> 31) ^ h;
    if (h >= 309) continue L_o;
  }
  label B_n:
  var q:double = ((h << 3) + 1050552)[0]:double;
  if (e > -1) goto B_p;
  p = p / q;
  goto B_b;
  label B_p:
  p = p * q;
  if (
    f64_reinterpret_i64(i64_reinterpret_f64(p) & 9223372036854775807L) !=
    inf) goto B_b;
  f[0] = 13;
  a.b = f_ef(b, f);
  a.a = 1;
  goto B_a;
  label B_m:
  f[0] = 13;
  a.b = f_ef(b, f);
  a.a = 1;
  goto B_a;
  label B_d:
  f_ua(a, b, c, d, e);
  goto B_a;
  label B_c:
  f[0] = 12;
  h = f_df(b, f);
  a.a = 1;
  a.b = h;
  goto B_a;
  label B_b:
  (a + 8)[0]:double = select_if(p, -(p), c);
  a.a = 0;
  label B_a:
  g_a = f + 16;
}

function f_ya(a:{ a:int, b:int, c:int, d:int }, b:int_ptr) {
  var f:int_ptr;
  var d:int;
  var e:int_ptr;
  var c:int_ptr = f_kk(a, b);
  if (f_fj(a)) goto B_c;
  d = a.a;
  if (f_pi(a)) goto B_e;
  b = d + b;
  a = f_lk(a, d);
  if (a != 0[269236]:int) goto B_d;
  if ((c[1] & 3) != 3) goto B_c;
  0[269234]:int = b;
  f_sg(a, b, c);
  return ;
  label B_e:
  if (eqz(f_fl(1076536, a - d, a = d + b + 16))) goto B_b;
  0[269238]:int = 0[269238]:int - a;
  return ;
  label B_d:
  if (d < 256) goto B_f;
  f_nc(a);
  goto B_c;
  label B_f:
  e = (a + 12)[0]:int;
  if (e == (f = (a + 8)[0]:int)) goto B_g;
  f[3] = e;
  e[2] = f;
  goto B_c;
  label B_g:
  0[269134]:int = 0[269134]:int & -2 << (d >> 3);
  label B_c:
  if (eqz(f_ei(c))) goto B_h;
  f_sg(a, b, c);
  goto B_a;
  label B_h:
  if (c == 0[269237]:int) goto B_j;
  if (c != 0[269236]:int) goto B_i;
  0[269236]:int = a;
  0[269234]:int = (b = 0[269234]:int + b);
  f_gh(a, b);
  return ;
  label B_j:
  0[269237]:int = a;
  0[269235]:int = (b = 0[269235]:int + b);
  a.b = b | 1;
  if (a != 0[269236]:int) goto B_b;
  0[269234]:int = 0;
  0[269236]:int = 0;
  return ;
  label B_i:
  d = f_ej(c);
  b = d + b;
  if (d < 256) goto B_l;
  f_nc(c);
  goto B_k;
  label B_l:
  e = (c + 12)[0]:int;
  if (e == (c = (c + 8)[0]:int)) goto B_m;
  c[3] = e;
  e[2] = c;
  goto B_k;
  label B_m:
  0[269134]:int = 0[269134]:int & -2 << (d >> 3);
  label B_k:
  f_gh(a, b);
  if (a != 0[269236]:int) goto B_a;
  0[269234]:int = b;
  label B_b:
  return ;
  label B_a:
  if (b < 256) goto B_n;
  f_lc(a, b);
  return ;
  label B_n:
  c = b >> 3;
  b = (c << 3) + 1076544;
  d = 0[269134]:int;
  if (eqz(d & (c = 1 << c))) goto B_p;
  c = b[2];
  goto B_o;
  label B_p:
  0[269134]:int = d | c;
  c = b;
  label B_o:
  b[2] = a;
  c[3] = a;
  a.d = b;
  a.c = c;
}

function f_za(a:long_ptr, b:int_ptr):long {
  var g:long;
  var h:long;
  var c:int = g_a - 80;
  g_a = c;
  var d:int = c + 64;
  d[0]:long = 0L;
  var e:long_ptr = c + 32;
  e[0] = (g = a[0]) ^ 7816392313619706465L;
  var f:long_ptr = c + 48;
  f[0] = (h = (a + 8)[0]:long) ^ 8387220255154660723L;
  a = c + 40;
  a[0] = h ^ 7237128888997146477L;
  c[7]:long = 0L;
  c[1]:long = g;
  c[3]:long = g ^ 8317987319222330741L;
  c[2]:long = h;
  f_na(c + 8, b[0], (b + 4)[0]:int);
  c[79]:byte = 255;
  f_na(c + 8, c + 79, 1);
  h = d[0]:uint;
  var i:long = c[7]:long;
  var j:long = f[0];
  var k:long = e[0];
  g = a[0];
  var l:long = c[3]:long;
  g_a = c + 80;
  i = j ^ (h = i | h << 56L);
  j = i << 16L ^ (i = i + k);
  l = j + ((k = g + l) << 32L);
  i = (l ^ h) + (g = (h = i + (g = g << 13L ^ k)) ^ g << 17L);
  g = i ^ g << 13L;
  k = g + (h = (j = j << 21L ^ l) + (h << 32L ^ 255L));
  g = k ^ g << 17L;
  j = g << 13L ^ (g = g + (i = (h = j << 16L ^ h) + (i << 32L)));
  k = j << 17L ^ (j = j + (i = (h = h << 21L ^ i) + (k << 32L)));
  i = k << 13L ^ k + (g = (h = h << 16L ^ i) + (g << 32L));
  j = i + (h = (g = h << 21L ^ g) + (j << 32L));
  return ((j ^ (g << 16L ^ h) << 21L) ^ i << 17L) ^ j << 32L;
}

function f_ab(a:{ a:int, b:int }, b:int, c:int, d:int, e:int, f:int, g:int) {
  var n:ubyte_ptr;
  var q:ubyte_ptr;
  var r:ubyte_ptr;
  var p:int;
  var i:int;
  var h:int;
  h = b[6]:int;
  i = h - f;
  if (i >= d) goto B_b;
  var j:int = b[3]:int;
  var k:int = select_if(j, f, j > f);
  var l:int = b[8]:int;
  var m:int = b[4]:int;
  var s:long = b[0]:long;
  loop L_c {
    if (eqz(s >> (n = c + i)[0] & 1L)) goto B_f;
    var o:int = select_if(j, select_if(l, j, j > l), g) + -1;
    p = o;
    loop L_i {
      if (p != -1) goto B_j;
      p = select_if(f, l, g);
      o = select_if(p, j, p > j);
      p = j;
      loop L_n {
        if (o != p) goto B_o;
        b[6]:int = i;
        if (eqz(g)) goto B_m;
        goto B_a;
        label B_o:
        if (k == p) goto B_l;
        if (i + p >= d) goto B_k;
        q = n + p;
        r = e + p;
        p = p + 1;
        if (r[0] == q[0]) continue L_n;
      }
      h = h - m;
      p = m;
      if (eqz(g)) goto B_e;
      goto B_d;
      label B_m:
      b[8]:int = f;
      goto B_a;
      label B_l:
      f_ne(k, f, 1055004);
      unreachable;
      label B_k:
      f_ne(select_if(d, p = i + j, d > p), d, 1055020);
      unreachable;
      label B_j:
      if (o >= f) goto B_h;
      q = i + p;
      if (q >= d) goto B_g;
      q = n + p;
      r = e + p;
      p = p + -1;
      if (r[0] == q[0]) continue L_i;
    }
    h = h - j + p + 1;
    p = f;
    if (eqz(g)) goto B_e;
    goto B_d;
    label B_h:
    f_ne(p, f, 1054972);
    unreachable;
    label B_g:
    f_ne(q, d, 1054988);
    unreachable;
    label B_f:
    b[6]:int = i;
    p = f;
    h = i;
    if (g) goto B_d;
    label B_e:
    b[8]:int = p;
    l = p;
    label B_d:
    i = h - f;
    if (i < d) continue L_c;
  }
  label B_b:
  b[6]:int = 0;
  a.a = 0;
  return ;
  label B_a:
  a.b = i;
  (a + 8)[0]:int = h;
  a.a = 1;
}

function f_bb(a:int, b:{ a:int, b:int }, c:int, d:long_ptr) {
  var i:ushort_ptr;
  var o:long_ptr;
  var n:int;
  var m:int;
  var e:int = g_a - 64;
  g_a = e;
  var f:int = c[2]:int;
  var g:int = c[1]:int;
  var h:int = c[0]:int;
  i = b.b;
  if (i) goto B_c;
  i = 0;
  goto B_b;
  label B_c:
  var j:int = b.a;
  loop L_d {
    c = i + 268;
    var k:int = i[201];
    var l:int = k * 24;
    m = -1;
    n = 0;
    loop L_g {
      if (l != n) goto B_h;
      m = k;
      goto B_f;
      label B_h:
      o = c[2]:int;
      var p:int = c[0]:int;
      m = m + 1;
      n = n + 24;
      c = c + 12;
      o = 
        select_if(
          -1,
          (o = select_if(p = f_ck(h, p, select_if(f, o, f < o)), f - o, p)) !=
          0,
          o < 0);
      if (o == 1) continue L_g;
    }
    if (eqz(o & 255)) goto B_e;
    label B_f:
    if (eqz(j)) goto B_b;
    j = j + -1;
    i = (i + (m << 2) + 408)[0]:int;
    continue L_d;
    label B_e:
  }
  if (eqz(g)) goto B_i;
  f_mi(h, g, 1);
  label B_i:
  a[0]:long = (c = i + n + -24)[0]:long;
  c[0]:long = d[0];
  (a + 16)[0]:long = (o = c + 16)[0];
  (a + 8)[0]:long = (c = c + 8)[0]:long;
  c[0]:long = (d + 8)[0]:long;
  o[0] = (d + 16)[0]:long;
  goto B_a;
  label B_b:
  (e + 28)[0]:int = m;
  (e + 8 + 16)[0]:int = i;
  e[8]:int = b;
  e[5]:int = 0;
  e[4]:int = f;
  e[3]:int = g;
  e[2]:int = h;
  (e + 40 + 16)[0]:long = (d + 16)[0]:long;
  (e + 40 + 8)[0]:long = (d + 8)[0]:long;
  e[5]:long = d[0];
  f_ja(e + 8, e + 40);
  a[0]:byte = 6;
  label B_a:
  g_a = e + 64;
}

function f_cb(a:long_ptr, b:int_ptr):int {
  var d:byte_ptr;
  var e:int;
  var f:int;
  var g:long;
  var c:int = g_a - 128;
  g_a = c;
  d = b[0];
  if (d & 16) goto B_e;
  if (d & 32) goto B_d;
  a = f_dc(a[0], 1, b);
  goto B_a;
  label B_e:
  g = a[0];
  a = 128;
  d = c + 128;
  loop L_h {
    if (a) goto B_i;
    a = 0;
    goto B_f;
    label B_i:
    (d + -1)[0]:byte =
      select_if(48, 87, (f = (e = i32_wrap_i64(g)) & 15) < 10) + f;
    if (g < 16L) goto B_j;
    d = d + -2;
    d[0] = select_if(48, 87, (f = e & 255) < 160) + (f >> 4);
    a = a + -2;
    f = g < 256L;
    g = g >> 8L;
    if (eqz(f)) continue L_h;
    goto B_g;
    label B_j:
  }
  a = a + -1;
  label B_g:
  if (a >= 129) goto B_c;
  label B_f:
  a = f_ca(b, 1, 1072633, 2, c + a, 128 - a);
  goto B_a;
  label B_d:
  g = a[0];
  a = 128;
  d = c + 128;
  loop L_m {
    if (a) goto B_n;
    a = 0;
    goto B_k;
    label B_n:
    (d + -1)[0]:byte =
      select_if(48, 55, (f = (e = i32_wrap_i64(g)) & 15) < 10) + f;
    if (g < 16L) goto B_o;
    d = d + -2;
    d[0] = select_if(48, 55, (f = e & 255) < 160) + (f >> 4);
    a = a + -2;
    f = g < 256L;
    g = g >> 8L;
    if (eqz(f)) continue L_m;
    goto B_l;
    label B_o:
  }
  a = a + -1;
  label B_l:
  if (a >= 129) goto B_b;
  label B_k:
  a = f_ca(b, 1, 1072633, 2, c + a, 128 - a);
  goto B_a;
  label B_c:
  f_rj(a, 128, a);
  unreachable;
  label B_b:
  f_rj(a, 128, a);
  unreachable;
  label B_a:
  g_a = c + 128;
  return a;
}

function f_db(a:int_ptr, b:long, c:long_ptr@4, d:int):int {
  var g:int;
  var h:int;
  var i:int;
  var f:long_ptr;
  var j:long_ptr@4;
  var e:int = g_a - 16;
  g_a = e;
  f = (a + 4)[0]:int;
  b = (f + (i = (g = a[0]) & (h = i32_wrap_i64(b))))[0]:long@1 &
      -9187201950435737472L;
  if (b != 0L) goto B_a;
  j = 8;
  loop L_b {
    i = i + j;
    j = j + 8;
    b = (f + (i = i & g))[0]:long@1 & -9187201950435737472L;
    if (eqz(b)) continue L_b;
  }
  label B_a:
  i = (f + (j = (i32_wrap_i64(ctz(b)) >> 3) + i & g))[0]:byte;
  if (i <= -1) goto B_c;
  i = 
    (f + (j = i32_wrap_i64(ctz(f[0] & -9187201950435737472L)) >> 3))[0]:ubyte;
  label B_c:
  i = i & 1;
  if (a[2]) goto B_d;
  if (eqz(i)) goto B_d;
  f_o(e, a, d);
  f = (a + 4)[0]:int;
  b = (f + (d = (g = a[0]) & h))[0]:long@1 & -9187201950435737472L;
  if (b != 0L) goto B_e;
  j = 8;
  loop L_f {
    d = d + j;
    j = j + 8;
    b = (f + (d = d & g))[0]:long@1 & -9187201950435737472L;
    if (eqz(b)) continue L_f;
  }
  label B_e:
  if (
    (f + (j = (i32_wrap_i64(ctz(b)) >> 3) + d & g))[0]:byte <= -1) goto B_d;
  j = i32_wrap_i64(ctz(f[0] & -9187201950435737472L)) >> 3;
  label B_d:
  (f + j)[0]:byte = (h = h >> 25);
  ((j + -8 & g) + f + 8)[0]:byte = h;
  a[2] = a[2] - i;
  a[3] = a[3] + 1;
  i = f + (0 - j) * 20;
  j = i + -20;
  (j + 16)[0]:int = (c + 16)[0]:int;
  (j + 8)[0]:long@4 = (c + 8)[0]:long@4;
  j[0] = c[0];
  g_a = e + 16;
  return i;
}

function f_eb(a:{ a:int, b:int }, b:byte_ptr, c:int, d:int, e:int, f:int, g:long, h:long, i:long) {
  var n:int;
  if (h <= i) goto B_g;
  if (h - i <= i) goto B_f;
  if (h - g <= g) goto B_h;
  if (h - (g << 1L) >= i << 1L) goto B_e;
  label B_h:
  if (g <= i) goto B_i;
  if (h - (i = g - i) <= i) goto B_d;
  label B_i:
  a.a = 0;
  return ;
  label B_g:
  a.a = 0;
  return ;
  label B_f:
  a.a = 0;
  return ;
  label B_e:
  if (d > c) goto B_c;
  a.b = d;
  a.a = b;
  (a + 8)[0]:short = e;
  return ;
  label B_d:
  if (d > c) goto B_b;
  var j:byte_ptr = b + d;
  var k:int = 0;
  var l:int = b;
  loop L_k {
    if (d == k) goto B_j;
    k = k + 1;
    var m:int = l + d;
    n = l + -1;
    l = n;
    if ((m + -1)[0]:ubyte == 57) continue L_k;
  }
  l = n + d;
  l[0]:byte = l[0]:ubyte + 1;
  if (d - k + 1 >= d) goto B_a;
  f_bk(l + 1, 48, k + -1);
  goto B_a;
  label B_j:
  if (d) goto B_m;
  k = 49;
  goto B_l;
  label B_m:
  b[0] = 49;
  k = 48;
  if (d == 1) goto B_l;
  k = 48;
  f_bk(b + 1, 48, d + -1);
  label B_l:
  e = (e << 16) + 65536 >> 16;
  if (e <= (f << 16) >> 16) goto B_a;
  if (d >= c) goto B_a;
  j[0] = k;
  d = d + 1;
  goto B_a;
  label B_c:
  f_sj(d, c, d);
  unreachable;
  label B_b:
  f_sj(d, c, d);
  unreachable;
  label B_a:
  if (d > c) goto B_n;
  a.b = d;
  a.a = b;
  (a + 8)[0]:short = e;
  return ;
  label B_n:
  f_sj(d, c, d);
  unreachable;
}

function f_fb(a:{ a:int, b:int }, b:int, c:int) {
  var g:int;
  var d:int = g_a - 128;
  g_a = d;
  f_kh(d + 8, b, c);
  (d + 24 + 8)[0]:int = (d + 8 + 8)[0]:int;
  (d + 44)[0]:int = 0;
  d[3]:long = d[1]:long;
  d[48]:byte = 128;
  d[9]:long@4 = 1L;
  f_h(d + 88, d + 24);
  if (d[22]:int) goto B_d;
  var e:long_ptr = d + 56 + 24;
  e[0] = (d + 88 + 32)[0]:long;
  var f:long_ptr = d + 56 + 16;
  f[0] = (d + 88 + 24)[0]:long;
  (d + 56 + 8)[0]:long = (d + 88 + 16)[0]:long;
  d[7]:long = (d + 88 + 8)[0]:long;
  b = d[8]:int;
  if (b >= (g = d[7]:int)) goto B_e;
  var h:int = d[6]:int;
  loop L_f {
    c = (h + b)[0]:ubyte + -9;
    if (c > 23) goto B_c;
    if (eqz(1 << c & 8388627)) goto B_c;
    d[8]:int = (b = b + 1);
    if (g != b) continue L_f;
  }
  label B_e:
  a.a = 0;
  (a + 8)[0]:long = d[7]:long;
  (a + 32)[0]:long = e[0];
  (a + 24)[0]:long = f[0];
  (a + 16)[0]:long = (d + 56 + 8)[0]:long;
  b = d[10]:int;
  if (eqz(b)) goto B_a;
  f_mi(d[9]:int, b, 1);
  goto B_a;
  label B_d:
  a.b = d[23]:int;
  a.a = 1;
  goto B_b;
  label B_c:
  d[22]:int = 19;
  b = f_df(d + 24, d + 88);
  a.a = 1;
  a.b = b;
  f_uc(d + 72);
  label B_b:
  b = d[10]:int;
  if (eqz(b)) goto B_a;
  f_mi(d[9]:int, b, 1);
  label B_a:
  g_a = d + 128;
}

function f_gb(a:int, b:int, c:int, d:int) {
  var g:int;
  var f:int;
  var h:int;
  var e:{ a:int, b:int, c:int, d:int, e:int, f:int, g:long } = g_a - 32;
  g_a = e;
  if (d) goto B_b;
  d = 0;
  a[1]:int = 0;
  (a + 12)[0]:long@4 = 0L;
  (a + 8)[0]:int = 1049336;
  goto B_a;
  label B_b:
  if (d < 8) goto B_d;
  if ((d & 536870911) != d) goto B_e;
  d = (-1 >> clz((d << 3) / 7 + -1)) + 1;
  goto B_c;
  label B_e:
  d = 1;
  f_mf(e + 24, 1);
  a[1]:long@4 = e.g;
  goto B_a;
  label B_d:
  d = select_if(4, 8, d < 4);
  label B_c:
  var i:long = i64_extend_i32_u(b) * i64_extend_i32_u(d);
  if (i32_wrap_i64(i >> 32L)) goto B_h;
  f = c + (b = i32_wrap_i64(i)) + -1;
  if (f < b) goto B_h;
  b = f & 0 - c;
  f = b + (g = d + 8);
  if (f < b) goto B_h;
  if (c) goto B_g;
  label B_h:
  f_mf(e, 1);
  d = e.b;
  b = e.a;
  goto B_f;
  label B_g:
  if (f < 0) goto B_j;
  if (f) goto B_l;
  h = c;
  if (c) goto B_k;
  goto B_i;
  label B_l:
  h = f_wh(f, c);
  if (eqz(h)) goto B_i;
  label B_k:
  (a + 8)[0]:int = f_bk(h + b, 255, g);
  a[1]:int = (b = d + -1);
  (a + 12)[0]:long@4 =
    i64_extend_i32_u(select_if(b, (d >> 3) * 7, b < 8));
  d = 0;
  goto B_a;
  label B_j:
  f_mf(e + 8, 1);
  d = e.d;
  b = e.c;
  goto B_f;
  label B_i:
  f_vg(e + 16, 1, f, c);
  d = e.f;
  b = e.e;
  label B_f:
  a[1]:int = b;
  (a + 8)[0]:int = d;
  d = 1;
  label B_a:
  a[0]:int = d;
  g_a = e + 32;
}

function f_hb(a:int_ptr, b:int_ptr):int {
  var f:int;
  var c:{ a:int, b:int } = g_a - 16;
  g_a = c;
  var d:int = 1;
  var e:int = b[6];
  if (call_indirect(e, 39, f = ((b + 28)[0]:int)[4]:int)) goto B_a;
  f_sb(c, a[0], 257);
  var g:int = (c + 12)[0]:ubyte;
  var h:int = (c + 8)[0]:int;
  b = c.a;
  var i:int = c.b;
  if (i == 1114112) goto B_d;
  loop L_e {
    a = b;
    d = 92;
    b = 1;
    br_table[B_b, B_h, B_f, B_i, ..B_b](a)
    label B_i:
    a = g & 255;
    g = 0;
    b = 3;
    d = 125;
    br_table[B_b, B_f, B_g, B_l, B_k, B_j, ..B_b](a)
    label B_l:
    g = 2;
    d = 123;
    goto B_f;
    label B_k:
    b = 3;
    d = 117;
    g = 3;
    goto B_f;
    label B_j:
    g = 4;
    d = 92;
    goto B_f;
    label B_h:
    b = 0;
    d = i;
    goto B_f;
    label B_g:
    g = select_if(2, 1, h);
    d = select_if(48, 87, (d = i >> (h << 2) & 15) < 10) + d;
    h = select_if(h + -1, 0, h);
    label B_f:
    if (eqz(call_indirect(e, d, f))) continue L_e;
    goto B_c;
  }
  label B_d:
  loop L_m {
    a = b;
    d = 92;
    b = 1;
    br_table[B_b, B_b, B_n, B_o, ..B_b](a)
    label B_o:
    a = g & 255;
    g = 0;
    b = 3;
    d = 125;
    br_table[B_b, B_n, B_p, B_q, B_r, B_s, ..B_b](a)
    label B_s:
    g = 4;
    d = 92;
    goto B_n;
    label B_r:
    b = 3;
    d = 117;
    g = 3;
    goto B_n;
    label B_q:
    g = 2;
    d = 123;
    goto B_n;
    label B_p:
    g = select_if(2, 1, h);
    d = (1114112 >> (h << 2) & 1) | 48;
    h = select_if(h + -1, 0, h);
    label B_n:
    if (eqz(call_indirect(e, d, f))) continue L_m;
  }
  label B_c:
  d = 1;
  goto B_a;
  label B_b:
  d = call_indirect(e, 39, f);
  label B_a:
  g_a = c + 16;
  return d;
}

function f_ib(a:long, b:int) {
  var c:short_ptr@1;
  var d:int;
  var e:int;
  var f:int;
  var g:int;
  var h:long;
  if (a >= 4294967296L) goto B_b;
  c = b;
  h = a;
  goto B_a;
  label B_b:
  c = b + -8;
  c[0] =
    (((g = 
         (f = 
            (e = 
               (d = i32_wrap_i64((h = a / 100000000L) * -100000000L + a)) / 10000) %
            10000) /
         100) <<
      1) +
     1066824)[0]:ushort@1;
  (b + -4)[0]:short@1 =
    (((e = ((d = d - e * 10000) & 65535) / 100) << 1) + 1066824)[0]:ushort@1;
  (b + -6)[0]:short@1 =
    (((f - g * 100 & 65535) << 1) + 1066824)[0]:ushort@1;
  (b + -2)[0]:short@1 =
    (((d - e * 100 & 65535) << 1) + 1066824)[0]:ushort@1;
  label B_a:
  b = i32_wrap_i64(h);
  if (b >= 10000) goto B_d;
  d = b;
  goto B_c;
  label B_d:
  c = c + -4;
  loop L_e {
    c[0] =
      (((f = (e = (d = b / 10000) * -10000 + b) / 100) << 1) + 1066824)[0]:ushort@1;
    (c + 2)[0]:short@1 = ((e - f * 100 << 1) + 1066824)[0]:ushort@1;
    c = c + -4;
    e = b > 99999999;
    b = d;
    if (e) continue L_e;
  }
  c = c + 4;
  label B_c:
  if (d > 99) goto B_g;
  b = d;
  goto B_f;
  label B_g:
  c = c + -2;
  c[0] =
    (((d - (b = (d & 65535) / 100) * 100 & 65535) << 1) + 1066824)[0]:ushort@1;
  label B_f:
  if (b > 9) goto B_h;
  (c + -1)[0]:byte = b + 48;
  return ;
  label B_h:
  (c + -2)[0]:short@1 = ((b << 1) + 1066824)[0]:ushort@1;
}

function f_jb(a:int, b:int_ptr, c:int, d:int, e:int):int {
  var l:int;
  var m:int;
  var n:int;
  var k:uint_ptr;
  var j:int;
  var f:int = b + (c << 2);
  if (eqz(e)) goto B_b;
  var g:int = e + 1;
  var h:int = e << 2;
  var i:int = 0;
  j = 0;
  loop L_c {
    k = a + (i << 2);
    loop L_d {
      l = i;
      c = k;
      if (b == f) goto B_a;
      k = c + 4;
      i = l + 1;
      m = b[0];
      n = b + 4;
      b = n;
      if (eqz(m)) continue L_d;
    }
    var o:int = select_if(l, 40, l < 40) + -40;
    var p:long = i64_extend_i32_u(m);
    var q:long = 0L;
    b = 0;
    m = h;
    k = d;
    loop L_h {
      if (o == b) goto B_g;
      c[0]:int = (q = q + c[0]:uint + k[0] * p);
      q = q >> 32L;
      c = c + 4;
      b = b + -1;
      k = k + 4;
      m = m + -4;
      if (m) continue L_h;
    }
    b = e;
    c = i32_wrap_i64(q);
    if (c) goto B_f;
    goto B_e;
    label B_g:
    f_ne((b ^ -1) + i, 40, 1075496);
    unreachable;
    label B_f:
    b = l + e;
    if (b > 39) goto B_i;
    a[b]:int = c;
    b = g;
    goto B_e;
    label B_i:
    f_ne(b, 40, 1075496);
    unreachable;
    label B_e:
    b = b + l;
    j = select_if(b, j, j < b);
    b = n;
    continue L_c;
  }
  label B_b:
  j = 0;
  c = 0;
  loop L_j {
    if (b == f) goto B_a;
    c = c + 1;
    k = b[0];
    l = b + 4;
    b = l;
    if (eqz(k)) continue L_j;
    b = c + -1;
    j = select_if(b, j, j < b);
    b = l;
    continue L_j;
  }
  label B_a:
  return j;
}

function f_kb(a:{ a:int, b:int }, b:int_ptr, c:int, d:long) {
  var h:int_ptr;
  var i:int;
  var l:double;
  var e:int_ptr = g_a - 16;
  g_a = e;
  var f:int = 0;
  var g:int = (b + 4)[0]:int;
  if (g <= (i = (h = b + 8)[0])) goto B_d;
  var j:int = i + 1;
  var k:int = g - i;
  g = b[0] + i;
  f = 0;
  loop L_e {
    i = (g + f)[0]:ubyte;
    if ((i + -48 & 255) < 10) goto B_f;
    if (i == 46) goto B_c;
    if (i == 69) goto B_g;
    if (i != 101) goto B_d;
    label B_g:
    f_ua(a, b, c, d, f);
    goto B_a;
    label B_f:
    h[0] = j + f;
    if (k != (f = f + 1)) continue L_e;
  }
  f = k;
  label B_d:
  l = f64_convert_i64_u(d);
  h = f + (h = f >> 31) ^ h;
  if (h < 309) goto B_i;
  loop L_j {
    if (l == 0.0) goto B_b;
    if (f > -1) goto B_h;
    l = 
      l /
      100000000000000001097906362944045541740492309677311846336810682903157585404911491537163328978494688899061249669721172515611590283743140088328307009198146046031271664502933027185697489699588559043338384466165001178426897626212945177628091195786707458122783970171784415105291802893207873272974885715430223118336.0;
    f = f + 308;
    h = f + (h = f >> 31) ^ h;
    if (h >= 309) continue L_j;
  }
  label B_i:
  var m:double = ((h << 3) + 1050552)[0]:double;
  if (f > -1) goto B_k;
  l = l / m;
  goto B_b;
  label B_k:
  l = l * m;
  if (
    f64_reinterpret_i64(i64_reinterpret_f64(l) & 9223372036854775807L) !=
    inf) goto B_b;
  e[0] = 13;
  a.b = f_ef(b, e);
  a.a = 1;
  goto B_a;
  label B_h:
  e[0] = 13;
  a.b = f_ef(b, e);
  a.a = 1;
  goto B_a;
  label B_c:
  f_xa(a, b, c, d, f);
  goto B_a;
  label B_b:
  (a + 8)[0]:double = select_if(l, -(l), c);
  a.a = 0;
  label B_a:
  g_a = e + 16;
}

function f_lb(a:int, b:int, c:int_ptr, d:int, e:long_ptr@4) {
  var n:int;
  var q:long;
  var f:int = g_a - 48;
  g_a = f;
  f[1]:int = d;
  f[0]:int = c;
  var o:long = f_za(b, f);
  var g:int = (b + 20)[0]:int;
  var h:int = g + -20;
  var i:int_ptr = b + 16;
  var j:int = i[0];
  var k:int = j & i32_wrap_i64(o);
  var p:long = (o >> 25L & 127L) * 72340172838076673L;
  c = f[1]:int;
  var l:int = f[0]:int;
  var m:int = 0;
  loop L_b {
    q = (g + k)[0]:long@1;
    var r:long = q ^ p;
    r = ((r ^ -1L) & r + -72340172838076673L) & -9187201950435737472L;
    if (eqz(r)) goto B_c;
    loop L_e {
      if (
        c !=
        ((d = h + (n = 0 - ((i32_wrap_i64(ctz(r)) >> 3) + k & j)) * 20) + 4)[0]:int) goto B_f;
      if (eqz(f_ck(l, d[0]:int, c))) goto B_d;
      label B_f:
      r = r + -1L & r;
      if (eqz(r)) goto B_c;
      continue L_e;
    }
    label B_d:
    c = g + n * 20;
    d = c + -12;
    r = d[0]:long@4;
    d[0]:long@4 = e[0];
    c = c + -4;
    d = c[0];
    c[0] = (e + 8)[0]:int;
    a[0]:long@4 = r;
    (a + 8)[0]:int = d;
    goto B_a;
    label B_c:
    if (eqz(eqz((q & q << 1L) & -9187201950435737472L))) goto B_g;
    k = k + (m = m + 8) & j;
    continue L_b;
    label B_g:
  }
  (f + 40)[0]:int = (e + 8)[0]:int;
  f[7]:int = c;
  f[6]:int = l;
  f[4]:long = e[0];
  f_db(i, o, f + 24, b);
  a[0]:int = 0;
  label B_a:
  g_a = f + 48;
}

function f_mb(a:int_ptr, b:{ a:int, b:int }):int {
  var e:int;
  var c:int;
  if (b < 9) goto B_e;
  if (f_nh(16, 8) > b) goto B_d;
  goto B_c;
  label B_e:
  c = f_e(a);
  goto B_b;
  label B_d:
  b = f_nh(16, 8);
  label B_c:
  var d:int = f_bl();
  c = 0;
  d = (d - f_nh(d, 8) + f_nh(20, 8) + f_nh(16, 8) + -65544 & -9) + -3;
  if (
    select_if(d, e = 0 - (f_nh(16, 8) << 2), e > d) - b <= a) goto B_b;
  d = 
    f_e(b + (e = f_nh(select_if(16, a + 4, f_nh(16, 8) + -5 > a), 8)) +
        f_nh(16, 8) +
        -4);
  if (eqz(d)) goto B_b;
  a = f_nk(d);
  c = b + -1;
  if (c & d) goto B_g;
  b = a;
  goto B_f;
  label B_g:
  c = f_nk(c + d & 0 - b);
  d = f_nh(16, 8);
  d = f_ej(a) - (c = (b = c + select_if(0, b, c - a > d)) - a);
  if (f_pi(a)) goto B_h;
  f_ig(b, d);
  f_ig(a, c);
  f_ya(a, c);
  goto B_f;
  label B_h:
  a = a[0];
  b.b = d;
  b.a = a + c;
  label B_f:
  if (f_pi(b)) goto B_a;
  a = f_ej(b);
  if (a <= f_nh(16, 8) + e) goto B_a;
  c = f_kk(b, e);
  f_ig(b, e);
  f_ig(c, a = a - e);
  f_ya(c, a);
  goto B_a;
  label B_b:
  return c;
  label B_a:
  a = f_mk(b);
  f_pi(b);
  return a;
}

function f_nb(a:{ a:int, b:int }, b:ubyte_ptr, c:int, d:int, e:int, f:int, g:int) {
  var h:int;
  if (eqz(c)) goto B_e;
  if (b[0] < 49) goto B_d;
  if (g < 4) goto B_c;
  h = (d << 16) >> 16;
  if (h < 1) goto B_g;
  f[1]:int = b;
  g = 2;
  f[0]:short = 2;
  d = d & 65535;
  if (d >= c) goto B_f;
  f[12]:short = 2;
  f[6]:short = 2;
  f[2]:int = d;
  (f + 32)[0]:int = (c = c - d);
  (f + 28)[0]:int = b + d;
  (f + 20)[0]:int = 1;
  (f + 16)[0]:int = 1072162;
  g = 3;
  if (c >= e) goto B_a;
  e = e - c;
  goto B_b;
  label B_g:
  f[12]:short = 2;
  f[6]:short = 0;
  f[2]:int = 2;
  f[1]:int = 1072160;
  f[0]:short = 2;
  (f + 32)[0]:int = c;
  (f + 28)[0]:int = b;
  (f + 16)[0]:int = (b = 0 - h);
  g = 3;
  if (e <= c) goto B_a;
  c = e - c;
  if (c <= b) goto B_a;
  e = c + h;
  goto B_b;
  label B_f:
  f[6]:short = 0;
  f[2]:int = c;
  (f + 16)[0]:int = d - c;
  if (eqz(e)) goto B_a;
  f[12]:short = 2;
  (f + 32)[0]:int = 1;
  (f + 28)[0]:int = 1072162;
  goto B_b;
  label B_e:
  f_rf(1071836, 33, 1072040);
  unreachable;
  label B_d:
  f_rf(1072056, 33, 1072092);
  unreachable;
  label B_c:
  f_rf(1072108, 34, 1072144);
  unreachable;
  label B_b:
  f[18]:short = 0;
  (f + 40)[0]:int = e;
  g = 4;
  label B_a:
  a.b = g;
  a.a = f;
}

function f_ob(a:int, b:int):int {
  var c:int = g_a - 64;
  g_a = c;
  br_table[B_f, B_e, B_d, B_c, ..B_f](a[0]:ubyte)
  label B_f:
  c[1]:int = (a + 4)[0]:int;
  a = f_wh(20, 1);
  if (eqz(a)) goto B_a;
  (a + 16)[0]:int@1 = d_calledOptionunwraponaNoneval[19928]:int@1;
  (a + 8)[0]:long@1 = d_calledOptionunwraponaNoneval[19920]:long@1;
  a[0]:long@1 = d_calledOptionunwraponaNoneval[19912]:long@1;
  c[3]:long@4 = 85899345940L;
  c[2]:int = a;
  (c + 40 + 20)[0]:int = 2;
  (c + 36)[0]:int = 49;
  c[11]:long@4 = 3L;
  c[10]:int = 1068240;
  c[7]:int = 50;
  c[14]:int = c + 24;
  c[8]:int = c + 4;
  c[6]:int = c + 8;
  a = f_pe(b, c + 40);
  b = c[3]:int;
  if (eqz(b)) goto B_b;
  var d:int = c[2]:int;
  if (eqz(d)) goto B_b;
  f_mi(d, b, 1);
  goto B_b;
  label B_e:
  a = a[1]:ubyte;
  (c + 60)[0]:int = 1;
  c[11]:long@4 = 1L;
  c[10]:int = 1067468;
  c[3]:int = 51;
  c[7]:int = ((a = ((a ^ 32) & 63) << 2) + 1068508)[0]:int;
  c[6]:int = (a + 1068764)[0]:int;
  c[14]:int = c + 8;
  c[2]:int = c + 24;
  a = f_pe(b, c + 40);
  goto B_b;
  label B_d:
  a = (a + 4)[0]:int;
  a = f_xj(a[0]:int, a[1]:int, b);
  goto B_b;
  label B_c:
  a = (a + 4)[0]:int;
  a = call_indirect(a[0]:int, b, (a[1]:int)[4]:int);
  label B_b:
  g_a = c + 64;
  return a;
  label B_a:
  f_mj(20, 1);
  return unreachable;
}

function f_pb(a:int, b:int, c:int, d:int, e:int, f:int, g:int) {
  var h:int = g_a - 112;
  g_a = h;
  h[3]:int = c;
  h[2]:int = b;
  h[5]:int = e;
  h[4]:int = d;
  br_table[B_d, B_c, B_b, ..B_d](a & 255)
  label B_d:
  h[6]:int = 1072417;
  a = 2;
  goto B_a;
  label B_c:
  h[6]:int = 1072415;
  a = 2;
  goto B_a;
  label B_b:
  h[6]:int = 1072408;
  a = 7;
  label B_a:
  h[7]:int = a;
  if (f[0]:int) goto B_e;
  (h + 56 + 20)[0]:int = 73;
  (h + 68)[0]:int = 73;
  (h + 88 + 20)[0]:int = 3;
  h[23]:long@4 = 4L;
  h[22]:int = 1072516;
  h[15]:int = 69;
  h[26]:int = h + 56;
  h[18]:int = h + 16;
  h[16]:int = h + 8;
  h[14]:int = h + 24;
  f_ag(h + 88, g);
  unreachable;
  label B_e:
  (h + 32 + 16)[0]:long = (f + 16)[0]:long@4;
  (h + 32 + 8)[0]:long = (f + 8)[0]:long@4;
  h[4]:long = f[0]:long@4;
  (h + 88 + 20)[0]:int = 4;
  (h + 84)[0]:int = 74;
  (h + 56 + 20)[0]:int = 73;
  (h + 68)[0]:int = 73;
  h[23]:long@4 = 4L;
  h[22]:int = 1072480;
  h[15]:int = 69;
  h[26]:int = h + 56;
  h[20]:int = h + 32;
  h[18]:int = h + 16;
  h[16]:int = h + 8;
  h[14]:int = h + 24;
  f_ag(h + 88, g);
  unreachable;
}

function f_qb(a:int, b:{ a:ubyte, b:ubyte }, c:int, d:int, e:int, f:{ a:ubyte, b:ubyte }, g:int):int {
  var n:int;
  var k:int;
  var l:int;
  var h:int = 1;
  if (eqz(c)) goto B_b;
  var i:int = b + (c << 1);
  var j:int = (a & 65280) >> 8;
  k = 0;
  l = a & 255;
  loop L_d {
    var m:int = b + 2;
    n = k + (c = b.b);
    b = b.a;
    if (b == j) goto B_e;
    if (b > j) goto B_b;
    k = n;
    b = m;
    if (m != i) continue L_d;
    goto B_b;
    label B_e:
    if (n < k) goto B_f;
    if (n > e) goto B_c;
    b = d + k;
    loop L_h {
      if (eqz(c)) goto B_g;
      c = c + -1;
      k = b.a;
      b = b + 1;
      if (k != l) continue L_h;
    }
    h = 0;
    goto B_a;
    label B_g:
    k = n;
    b = m;
    if (m != i) continue L_d;
    goto B_b;
    label B_f:
  }
  f_tj(k, n, c);
  unreachable;
  label B_c:
  f_sj(n, e, c);
  unreachable;
  label B_b:
  if (eqz(g)) goto B_a;
  l = f + g;
  b = a & 65535;
  h = 1;
  loop L_j {
    k = f + 1;
    c = f.a;
    n = (c << 24) >> 24;
    if (n < 0) goto B_l;
    f = k;
    goto B_k;
    label B_l:
    if (k == l) goto B_i;
    c = (n & 127) << 8 | f.b;
    f = f + 2;
    label B_k:
    b = b - c;
    if (b < 0) goto B_a;
    h = h ^ 1;
    if (f != l) continue L_j;
    goto B_a;
  }
  label B_i:
  f_rf(1072240, 43, 1073972);
  unreachable;
  label B_a:
  return h & 1;
}

function f_rb(a:int, b:int, c:int, d:int, e:int_ptr):int {
  var i:int;
  var j:int;
  var f:int = g_a - 64;
  g_a = f;
  var g:int = 1;
  if (a[4]:ubyte) goto B_a;
  var h:int = a[5]:ubyte;
  i = a[0]:int;
  j = i[0]:int;
  if (j & 4) goto B_b;
  g = 1;
  if (call_indirect(i[6]:int,
                    select_if(1072601, 1072603, h = h & 255),
                    select_if(2, 3, h),
                    ((i + 28)[0]:int)[3]:int)) goto B_a;
  g = 1;
  if (call_indirect(i[6]:int, b, c, (i[7]:int)[3]:int)) goto B_a;
  g = 1;
  if (call_indirect(i[6]:int, 1072548, 2, (i[7]:int)[3]:int)) goto B_a;
  g = call_indirect(d, i, e[3]);
  goto B_a;
  label B_b:
  if (h & 255) goto B_c;
  g = 1;
  if (
    call_indirect(i[6]:int, 1072596, 3, ((i + 28)[0]:int)[3]:int)) goto B_a;
  j = i[0]:int;
  label B_c:
  g = 1;
  f[23]:byte = 1;
  (f + 52)[0]:int = 1072568;
  (f + 16)[0]:int = f + 23;
  f[6]:int = j;
  f[1]:long = i[6]:long@4;
  var k:long = i[2]:long@4;
  var l:long = i[4]:long@4;
  f[56]:byte = i[32]:ubyte;
  f[7]:int = i[1]:int;
  f[5]:long = l;
  f[4]:long = k;
  f[12]:int = f + 8;
  if (f_ia(f + 8, b, c)) goto B_a;
  if (f_ia(f + 8, 1072548, 2)) goto B_a;
  if (call_indirect(d, f + 24, e[3])) goto B_a;
  g = call_indirect(f[12]:int, 1072599, 2, (f[13]:int)[3]:int);
  label B_a:
  a[5]:byte = 1;
  a[4]:byte = g;
  g_a = f + 64;
  return a;
}

function f_sb(a:{ a:int, b:int }, b:int, c:int) {
  var f:long;
  var d:int = 48;
  var e:int = 2;
  br_table[B_a, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_h, B_f, B_c, B_c, B_g, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_c, B_e, B_c, B_c, B_c, B_c, B_d, ..B_i](
    b)
  label B_i:
  d = 92;
  if (b == 92) goto B_b;
  goto B_c;
  label B_h:
  d = 116;
  goto B_b;
  label B_g:
  d = 114;
  goto B_b;
  label B_f:
  d = 110;
  goto B_b;
  label B_e:
  if (eqz(c & 65536)) goto B_c;
  d = 34;
  goto B_b;
  label B_d:
  if (eqz(c & 256)) goto B_c;
  d = 39;
  goto B_b;
  label B_c:
  d = b;
  if (eqz(c & 1)) goto B_j;
  if (eqz(f_tb(d))) goto B_j;
  f = i64_extend_i32_u(clz(b | 1) >> 2 ^ 7) | 21474836480L;
  e = 3;
  goto B_a;
  label B_j:
  if (b < 65536) goto B_n;
  if (b >= 131072) goto B_m;
  if (f_qb(d, 1074659, 42, 1074743, 192, 1074935, 438)) goto B_k;
  goto B_l;
  label B_n:
  if (eqz(f_qb(d, 1073988, 40, 1074068, 288, 1074356, 303))) goto B_l;
  goto B_k;
  label B_m:
  if (b > 917999) goto B_l;
  if ((b & 2097150) == 178206) goto B_l;
  if ((b & 2097120) == 173792) goto B_l;
  if (b + -177977 < 7) goto B_l;
  if (b + -183984 > -15) goto B_l;
  if (b + -194560 > -3104) goto B_l;
  if (b + -196608 > -1507) goto B_l;
  if (b + -917760 < -716213) goto B_k;
  label B_l:
  f = i64_extend_i32_u(clz(b | 1) >> 2 ^ 7) | 21474836480L;
  e = 3;
  goto B_a;
  label B_k:
  e = 1;
  label B_b:
  label B_a:
  a.b = d;
  a.a = e;
  (a + 8)[0]:long@4 = f;
}

function f_tb(a:int):int {
  var f:int;
  var b:int = a << 11;
  var c:int = 0;
  var d:int = 32;
  var e:int = 32;
  loop L_c {
    d = (d >> 1) + c;
    f = ((d << 2) + 1075652)[0]:int << 11;
    if (f < b) goto B_e;
    if (f == b) goto B_b;
    e = d;
    goto B_d;
    label B_e:
    c = d + 1;
    label B_d:
    d = e - c;
    if (e > c) continue L_c;
    goto B_a;
  }
  label B_b:
  c = d + 1;
  label B_a:
  if (c > 31) goto B_h;
  d = c << 2;
  e = 707;
  if (c == 31) goto B_i;
  e = (d + 1075656)[0]:int >> 21;
  label B_i:
  b = 0;
  f = c + -1;
  if (f > c) goto B_j;
  if (f >= 32) goto B_g;
  b = ((f << 2) + 1075652)[0]:int & 2097151;
  label B_j:
  if (eqz(e + ((c = (d + 1075652)[0]:int >> 21) ^ -1))) goto B_k;
  b = a - b;
  d = select_if(c, 707, c > 707);
  f = e + -1;
  e = 0;
  loop L_l {
    if (d == c) goto B_f;
    e = e + (c + 1075780)[0]:ubyte;
    if (e > b) goto B_k;
    if (f != (c = c + 1)) continue L_l;
  }
  c = f;
  label B_k:
  return c & 1;
  label B_h:
  f_ne(c, 32, 1075416);
  unreachable;
  label B_g:
  f_ne(f, 32, 1075448);
  unreachable;
  label B_f:
  f_ne(d, 707, 1075432);
  return unreachable;
}

function f_ub(a:{ a:int, b:int }, b:int_ptr, c:int, d:long, e:int) {
  var i:int;
  var k:int;
  var h:int;
  var l:double;
  var f:int_ptr = g_a - 16;
  g_a = f;
  var g:int_ptr = b + 8;
  h = g[0];
  if (h >= (i = (b + 4)[0]:int)) goto B_d;
  var j:int = b[0];
  loop L_f {
    k = (j + h)[0]:ubyte;
    if ((k + -48 & 255) >= 10) goto B_e;
    g[0] = (h = h + 1);
    if (i != h) continue L_f;
    goto B_d;
  }
  label B_e:
  if ((k | 32) == 101) goto B_c;
  label B_d:
  l = f64_convert_i64_u(d);
  h = e + (h = e >> 31) ^ h;
  if (h < 309) goto B_h;
  loop L_i {
    if (l == 0.0) goto B_b;
    if (e > -1) goto B_g;
    l = 
      l /
      100000000000000001097906362944045541740492309677311846336810682903157585404911491537163328978494688899061249669721172515611590283743140088328307009198146046031271664502933027185697489699588559043338384466165001178426897626212945177628091195786707458122783970171784415105291802893207873272974885715430223118336.0;
    e = e + 308;
    h = e + (h = e >> 31) ^ h;
    if (h >= 309) continue L_i;
  }
  label B_h:
  var m:double = ((h << 3) + 1050552)[0]:double;
  if (e > -1) goto B_j;
  l = l / m;
  goto B_b;
  label B_j:
  l = l * m;
  if (
    f64_reinterpret_i64(i64_reinterpret_f64(l) & 9223372036854775807L) !=
    inf) goto B_b;
  f[0] = 13;
  a.b = f_ef(b, f);
  a.a = 1;
  goto B_a;
  label B_g:
  f[0] = 13;
  a.b = f_ef(b, f);
  a.a = 1;
  goto B_a;
  label B_c:
  f_ua(a, b, c, d, e);
  goto B_a;
  label B_b:
  (a + 8)[0]:double = select_if(l, -(l), c);
  a.a = 0;
  label B_a:
  g_a = f + 16;
}

function f_vb(a:int_ptr, b:int):int {
  var d:int;
  var e:int_ptr;
  var c:int = g_a - 16;
  g_a = c;
  a = a[0];
  if (b < 128) goto B_e;
  c[3]:int = 0;
  if (b < 2048) goto B_d;
  if (b >= 65536) goto B_c;
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  goto B_b;
  label B_e:
  d = a[2];
  if (d != (a + 4)[0]:int) goto B_f;
  f_gd(a, d);
  d = a[2];
  label B_f:
  (a[0] + d)[0]:byte = b;
  a[2] = a[2] + 1;
  goto B_a;
  label B_d:
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_b;
  label B_c:
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  label B_b:
  if ((a + 4)[0]:int - (d = (e = a + 8)[0]) >= b) goto B_g;
  f_hd(a, d, b);
  d = e[0];
  label B_g:
  f_dk(a[0] + d, c + 12, b);
  e[0] = d + b;
  label B_a:
  g_a = c + 16;
  return 0;
}

function f_wb(a:int_ptr, b:int):int {
  var d:int;
  var e:int_ptr;
  var c:int = g_a - 16;
  g_a = c;
  a = a[0];
  if (b < 128) goto B_e;
  c[3]:int = 0;
  if (b < 2048) goto B_d;
  if (b >= 65536) goto B_c;
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  goto B_b;
  label B_e:
  d = a[2];
  if (d != (a + 4)[0]:int) goto B_f;
  f_id(a, d);
  d = a[2];
  label B_f:
  (a[0] + d)[0]:byte = b;
  a[2] = a[2] + 1;
  goto B_a;
  label B_d:
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_b;
  label B_c:
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  label B_b:
  if ((a + 4)[0]:int - (d = (e = a + 8)[0]) >= b) goto B_g;
  f_jd(a, d, b);
  d = e[0];
  label B_g:
  f_dk(a[0] + d, c + 12, b);
  e[0] = d + b;
  label B_a:
  g_a = c + 16;
  return 0;
}

function f_xb(a:int_ptr, b:int):int {
  var d:int;
  var e:int_ptr;
  var c:int = g_a - 16;
  g_a = c;
  if (b < 128) goto B_e;
  c[3]:int = 0;
  if (b < 2048) goto B_d;
  if (b >= 65536) goto B_c;
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  goto B_b;
  label B_e:
  d = a[2];
  if (d != (a + 4)[0]:int) goto B_f;
  f_gd(a, d);
  d = a[2];
  label B_f:
  (a[0] + d)[0]:byte = b;
  a[2] = a[2] + 1;
  goto B_a;
  label B_d:
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_b;
  label B_c:
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  label B_b:
  if ((a + 4)[0]:int - (d = (e = a + 8)[0]) >= b) goto B_g;
  f_hd(a, d, b);
  d = e[0];
  label B_g:
  f_dk(a[0] + d, c + 12, b);
  e[0] = d + b;
  label B_a:
  g_a = c + 16;
  return 0;
}

function f_yb(a:int_ptr, b:int):int {
  var d:int;
  var e:int_ptr;
  var c:int = g_a - 16;
  g_a = c;
  if (b < 128) goto B_e;
  c[3]:int = 0;
  if (b < 2048) goto B_d;
  if (b >= 65536) goto B_c;
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  goto B_b;
  label B_e:
  d = a[2];
  if (d != (a + 4)[0]:int) goto B_f;
  f_id(a, d);
  d = a[2];
  label B_f:
  (a[0] + d)[0]:byte = b;
  a[2] = a[2] + 1;
  goto B_a;
  label B_d:
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_b;
  label B_c:
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  label B_b:
  if ((a + 4)[0]:int - (d = (e = a + 8)[0]) >= b) goto B_g;
  f_jd(a, d, b);
  d = e[0];
  label B_g:
  f_dk(a[0] + d, c + 12, b);
  e[0] = d + b;
  label B_a:
  g_a = c + 16;
  return 0;
}

function f_zb(a:int_ptr, b:int):int {
  var d:int;
  var e:int_ptr;
  var c:int = g_a - 16;
  g_a = c;
  if (b < 128) goto B_d;
  c[3]:int = 0;
  if (b >= 2048) goto B_c;
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_b;
  label B_d:
  d = a[2];
  if (d != (a + 4)[0]:int) goto B_e;
  f_ld(a, d);
  d = a[2];
  label B_e:
  a[2] = d + 1;
  (a[0] + d)[0]:byte = b;
  goto B_a;
  label B_c:
  if (b < 65536) goto B_f;
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  goto B_b;
  label B_f:
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  label B_b:
  if ((a + 4)[0]:int - (d = (e = a + 8)[0]) >= b) goto B_g;
  f_kd(a, d, b);
  d = e[0];
  label B_g:
  f_dk(a[0] + d, c + 12, b);
  e[0] = d + b;
  label B_a:
  g_a = c + 16;
  return 0;
}

function f_ac(a:{ a:int, b:int }, b:int_ptr) {
  var f:int;
  var e:int;
  var g:int;
  var h:int;
  var c:{ a:int, b:int } = g_a - 16;
  g_a = c;
  var d:int_ptr = b + 8;
  e = d[0];
  if (e >= (f = (b + 4)[0]:int)) goto B_c;
  g = b[0];
  loop L_d {
    h = (g + e)[0]:ubyte + -9;
    if (h > 25) goto B_f;
    if (1 << h & 8388627) goto B_e;
    if (h == 25) goto B_b;
    label B_f:
    a.b = f_gf(f(b, c, 1049112), b);
    a.a = 1;
    goto B_a;
    label B_e:
    d[0] = (e = e + 1);
    if (f != e) continue L_d;
  }
  label B_c:
  c.a = 5;
  e = f_df(b, c);
  a.a = 1;
  a.b = e;
  goto B_a;
  label B_b:
  (b + 20)[0]:int = 0;
  (b + 8)[0]:int = e + 1;
  f_r(c, b, b + 12);
  if (c.a) goto B_h;
  e = (c + 12)[0]:int;
  if (e < 0) goto B_j;
  h = (c + 8)[0]:int;
  if (e) goto B_i;
  g = 1;
  goto B_g;
  label B_j:
  f_zf();
  unreachable;
  label B_i:
  g = f_wh(e, 1);
  if (g) goto B_g;
  f_mj(e, 1);
  unreachable;
  label B_h:
  a.b = c.b;
  a.a = 1;
  goto B_a;
  label B_g:
  h = f_dk(g, h, e);
  (a + 12)[0]:int = e;
  (a + 8)[0]:int = e;
  a.b = h;
  a.a = 0;
  label B_a:
  g_a = c + 16;
}

function f_bc(a:{ a:byte, b:byte }, b:ubyte_ptr, c:int, d:int) {
  var i:int;
  var j:int;
  var k:int;
  var l:int;
  var f:int;
  var e:int_ptr = g_a - 16;
  g_a = e;
  if (d < c) goto B_c;
  if (d > c) goto B_b;
  f = 1;
  c = 0;
  var g:int = 1;
  if (eqz(d)) goto B_d;
  var h:int = d & 3;
  if (d + -1 >= 3) goto B_f;
  c = 0;
  g = 1;
  goto B_e;
  label B_f:
  d = d & -4;
  g = 1;
  c = 0;
  loop L_g {
    c = 
      select_if(0,
                select_if(1,
                          select_if(2,
                                    select_if(3, c + 4, i = b[0] == 10),
                                    j = (b + 1)[0]:ubyte == 10),
                          k = (b + 2)[0]:ubyte == 10),
                l = (b + 3)[0]:ubyte == 10);
    g = g + i + j + k + l;
    b = b + 4;
    d = d + -4;
    if (d) continue L_g;
  }
  label B_e:
  if (eqz(h)) goto B_d;
  loop L_h {
    c = select_if(0, c + 1, d = b[0] == 10);
    b = b + 1;
    g = g + d;
    h = h + -1;
    if (h) continue L_h;
  }
  label B_d:
  e[0] = 4;
  (a + 4)[0]:int = f_tf(e, g, c);
  goto B_a;
  label B_c:
  a.b = (b + d)[0]:ubyte;
  f = 0;
  goto B_a;
  label B_b:
  f_sj(d, c, 1053316);
  unreachable;
  label B_a:
  a.a = f;
  g_a = e + 16;
}

function f_cc(a:int, b:int, c:int):int {
  var e:int;
  var d:byte_ptr;
  var h:int;
  var i:int;
  var g:ubyte_ptr;
  var j:int;
  var f:int_ptr;
  if (c > 15) goto B_b;
  d = a;
  goto B_a;
  label B_b:
  f = a + (e = 0 - a & 3);
  if (eqz(e)) goto B_c;
  d = a;
  g = b;
  loop L_d {
    d[0] = g[0];
    g = g + 1;
    d = d + 1;
    if (d < f) continue L_d;
  }
  label B_c:
  d = f + (i = (h = c - e) & -4);
  j = b + e;
  if (eqz(j & 3)) goto B_f;
  if (i < 1) goto B_e;
  g = j << 3;
  c = g & 24;
  var k:int_ptr = j & -4;
  b = k + 4;
  e = 0 - g & 24;
  g = k[0];
  loop L_g {
    f[0] = g >> c | (g = b[0]:int) << e;
    b = b + 4;
    f = f + 4;
    if (f < d) continue L_g;
    goto B_e;
  }
  label B_f:
  if (i < 1) goto B_e;
  b = j;
  loop L_h {
    f[0] = b[0]:int;
    b = b + 4;
    f = f + 4;
    if (f < d) continue L_h;
  }
  label B_e:
  c = h & 3;
  b = j + i;
  label B_a:
  if (eqz(c)) goto B_i;
  f = d + c;
  loop L_j {
    d[0] = b[0]:ubyte;
    b = b + 1;
    d = d + 1;
    if (d < f) continue L_j;
  }
  label B_i:
  return a;
}

function f_dc(a:long, b:int, c:int):int {
  var g:int;
  var h:int;
  var i:long;
  var f:int;
  var d:int = g_a - 48;
  g_a = d;
  var e:int = 39;
  if (a >= 10000L) goto B_b;
  i = a;
  goto B_a;
  label B_b:
  e = 39;
  loop L_c {
    f = d + 9 + e;
    (f + -4)[0]:short@1 =
      (((h = ((g = i32_wrap_i64(a - (i = a / 10000L) * 10000L)) & 65535) / 100) <<
        1) +
       1072635)[0]:ushort@1;
    (f + -2)[0]:short@1 =
      (((g - h * 100 & 65535) << 1) + 1072635)[0]:ushort@1;
    e = e + -4;
    f = a > 99999999L;
    a = i;
    if (f) continue L_c;
  }
  label B_a:
  f = i32_wrap_i64(i);
  if (f <= 99) goto B_d;
  (d + 9 + (e = e + -2))[0]:short@1 =
    ((((f = i32_wrap_i64(i)) - (f = (f & 65535) / 100) * 100 & 65535) << 1) +
     1072635)[0]:ushort@1;
  label B_d:
  if (f < 10) goto B_f;
  (d + 9 + (e = e + -2))[0]:short@1 = ((f << 1) + 1072635)[0]:ushort@1;
  goto B_e;
  label B_f:
  (d + 9 + (e = e + -1))[0]:byte = f + 48;
  label B_e:
  e = f_ca(c, b, 1072168, 0, d + 9 + e, 39 - e);
  g_a = d + 48;
  return e;
}

function f_ec(a:ubyte_ptr, b:int_ptr):int {
  var d:int;
  var e:int;
  var c:int = g_a - 128;
  g_a = c;
  d = b[0];
  if (d & 16) goto B_e;
  if (d & 32) goto B_d;
  a = f_dc(a[0], 1, b);
  goto B_a;
  label B_e:
  d = a[0];
  a = 0;
  loop L_f {
    (c + a + 127)[0]:byte = select_if(48, 87, (e = d & 15) < 10) + e;
    a = a + -1;
    e = d & 255;
    d = e >> 4;
    if (e > 15) continue L_f;
  }
  d = a + 128;
  if (d >= 129) goto B_c;
  a = f_ca(b, 1, 1072633, 2, c + a + 128, 0 - a);
  goto B_a;
  label B_d:
  d = a[0];
  a = 0;
  loop L_g {
    (c + a + 127)[0]:byte = select_if(48, 55, (e = d & 15) < 10) + e;
    a = a + -1;
    e = d & 255;
    d = e >> 4;
    if (e > 15) continue L_g;
  }
  d = a + 128;
  if (d >= 129) goto B_b;
  a = f_ca(b, 1, 1072633, 2, c + a + 128, 0 - a);
  goto B_a;
  label B_c:
  f_rj(d, 128, a);
  unreachable;
  label B_b:
  f_rj(d, 128, a);
  unreachable;
  label B_a:
  g_a = c + 128;
  return a;
}

function f_fc(a:int, b:int_ptr):int {
  var d:int;
  var e:int;
  var c:int = g_a - 128;
  g_a = c;
  d = b[0];
  if (d & 16) goto B_e;
  if (d & 32) goto B_d;
  a = f_dc(a[0]:uint, 1, b);
  goto B_a;
  label B_e:
  a = a[0]:int;
  d = 0;
  loop L_f {
    (c + d + 127)[0]:byte = select_if(48, 87, (e = a & 15) < 10) + e;
    d = d + -1;
    e = a > 15;
    a = a >> 4;
    if (e) continue L_f;
  }
  a = d + 128;
  if (a >= 129) goto B_c;
  a = f_ca(b, 1, 1072633, 2, c + d + 128, 0 - d);
  goto B_a;
  label B_d:
  a = a[0]:int;
  d = 0;
  loop L_g {
    (c + d + 127)[0]:byte = select_if(48, 55, (e = a & 15) < 10) + e;
    d = d + -1;
    e = a > 15;
    a = a >> 4;
    if (e) continue L_g;
  }
  a = d + 128;
  if (a >= 129) goto B_b;
  a = f_ca(b, 1, 1072633, 2, c + d + 128, 0 - d);
  goto B_a;
  label B_c:
  f_rj(a, 128, a);
  unreachable;
  label B_b:
  f_rj(a, 128, a);
  unreachable;
  label B_a:
  g_a = c + 128;
  return a;
}

function f_gc(a:ubyte_ptr, b:int, c:int, d:long_ptr@4):int {
  var h:int;
  var i:int;
  var j:int;
  var k:int;
  var g:int;
  var e:long_ptr = g_a - 16;
  g_a = e;
  if (c > b) goto B_a;
  if (c) goto B_c;
  c = 1;
  b = 0;
  goto B_b;
  label B_c:
  var f:int = c & 3;
  if (c + -1 >= 3) goto B_e;
  b = 0;
  c = 1;
  goto B_d;
  label B_e:
  g = c & -4;
  c = 1;
  b = 0;
  loop L_f {
    b = 
      select_if(0,
                select_if(1,
                          select_if(2,
                                    select_if(3, b + 4, h = a[0] == 10),
                                    i = (a + 1)[0]:ubyte == 10),
                          j = (a + 2)[0]:ubyte == 10),
                k = (a + 3)[0]:ubyte == 10);
    c = c + h + i + j + k;
    a = a + 4;
    g = g + -4;
    if (g) continue L_f;
  }
  label B_d:
  if (eqz(f)) goto B_b;
  loop L_g {
    b = select_if(0, b + 1, g = a[0] == 10);
    a = a + 1;
    c = c + g;
    f = f + -1;
    if (f) continue L_g;
  }
  label B_b:
  (e + 8)[0]:int = (d + 8)[0]:int;
  e[0] = d[0];
  a = f_tf(e, c, b);
  g_a = e + 16;
  return a;
  label B_a:
  f_sj(c, b, 1053316);
  return unreachable;
}

function f_hc(a:int, b:int, c:int_ptr):int {
  var e:int;
  var g:int;
  var h:int;
  var f:int;
  var d:int = g_a - 64;
  g_a = d;
  if (eqz(a[8]:ubyte)) goto B_b;
  e = a[1]:int;
  f = 1;
  goto B_a;
  label B_b:
  e = a[1]:int;
  g = a[0]:int;
  h = g[0]:int;
  if (h & 4) goto B_c;
  f = 1;
  if (call_indirect(g[6]:int,
                    select_if(1072601, 1072611, e),
                    select_if(2, 1, e),
                    ((g + 28)[0]:int)[3]:int)) goto B_a;
  f = call_indirect(b, g, c[3]);
  goto B_a;
  label B_c:
  if (e) goto B_d;
  if (
    eqz(call_indirect(g[6]:int, 1072609, 2, ((g + 28)[0]:int)[3]:int))) goto B_e;
  f = 1;
  e = 0;
  goto B_a;
  label B_e:
  h = g[0]:int;
  label B_d:
  f = 1;
  d[23]:byte = 1;
  (d + 52)[0]:int = 1072568;
  (d + 16)[0]:int = d + 23;
  d[6]:int = h;
  d[1]:long = g[6]:long@4;
  var i:long = g[2]:long@4;
  var j:long = g[4]:long@4;
  d[56]:byte = g[32]:ubyte;
  d[7]:int = g[1]:int;
  d[5]:long = j;
  d[4]:long = i;
  d[12]:int = d + 8;
  if (call_indirect(b, d + 24, c[3])) goto B_a;
  f = call_indirect(d[12]:int, 1072599, 2, (d[13]:int)[3]:int);
  label B_a:
  a[8]:byte = f;
  a[1]:int = e + 1;
  g_a = d + 64;
  return a;
}

function f_ic(a:{ a:int, b:int, c:int }, b:{ a:int, b:int, c:int }) {
  var e:int;
  var f:ushort_ptr;
  var g:int;
  var d:int;
  var c:int = b.a;
  d = b.c;
  if (d >= (e = b.b)[201]:ushort) goto B_b;
  f = e;
  g = c;
  goto B_a;
  label B_b:
  loop L_c {
    f = e[66]:int;
    if (f) goto B_e;
    f = 0;
    goto B_d;
    label B_e:
    g = c + 1;
    d = e[200]:ushort;
    label B_d:
    c = select_if(456, 408, c);
    if (eqz(c)) goto B_f;
    f_mi(e, c, 8);
    label B_f:
    if (eqz(f)) goto B_g;
    c = g;
    e = f;
    if (d < f[201]) goto B_a;
    continue L_c;
    label B_g:
  }
  f_rf(1048576, 43, 1049080);
  unreachable;
  label B_a:
  var h:int = d + 1;
  if (g) goto B_i;
  e = f;
  goto B_h;
  label B_i:
  e = (f + (h << 2) + 408)[0]:int;
  h = 0;
  c = g + -1;
  if (eqz(c)) goto B_h;
  var i:int = g + -2;
  var j:int = c & 7;
  if (eqz(j)) goto B_j;
  loop L_k {
    c = c + -1;
    e = e[102]:int;
    j = j + -1;
    if (j) continue L_k;
  }
  label B_j:
  if (i < 7) goto B_h;
  loop L_l {
    e = 
      (((((((e[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
    c = c + -8;
    if (c) continue L_l;
  }
  label B_h:
  a.c = d;
  a.b = f;
  a.a = g;
  b.c = h;
  b.b = e;
  b.a = 0;
}

function f_jc(a:int, b:int, c:int, d:int) {
  var e:int = g_a - 16;
  g_a = e;
  if (eqz(d)) goto B_b;
  var f:int = b[0]:int;
  if (eqz(f_ih(f, c, d))) goto B_b;
  loop L_c {
    b = f_wh(9, 1);
    if (eqz(b)) goto B_f;
    (b + 8)[0]:byte = d_calledOptionunwraponaNoneval[5648]:ubyte;
    b[0]:long@1 = d_calledOptionunwraponaNoneval[5640]:long@1;
    var g:int = f_wh(12, 4);
    if (eqz(g)) goto B_e;
    g[1]:long@4 = 38654705673L;
    g[0]:int = b;
    f_wf(e + 8, 39, g, 1053956);
    if (e[8]:ubyte == 4) goto B_b;
    b = e[3]:int;
    var h:int = e[2]:int;
    g = h >> 8;
    br_table[B_l, B_j, B_k, B_i, ..B_l](h & 255)
    label B_l:
    g = f_yk(b) & 255;
    goto B_j;
    label B_k:
    g = b[8]:ubyte;
    label B_j:
    if ((g & 255) != 35) goto B_h;
    goto B_d;
    label B_i:
    if (b[8]:ubyte == 35) goto B_g;
    label B_h:
    a[0]:int = h;
    (a + 4)[0]:int = b;
    goto B_a;
    label B_g:
    call_indirect(b[0]:int, (b[1]:int)[0]:int);
    g = b[1]:int;
    h = g[1]:int;
    if (eqz(h)) goto B_m;
    f_mi(b[0]:int, h, g[2]:int);
    label B_m:
    f_mi(b, 12, 4);
    goto B_d;
    label B_f:
    f_mj(9, 1);
    unreachable;
    label B_e:
    f_mj(12, 4);
    unreachable;
    label B_d:
    if (f_ih(f, c, d)) continue L_c;
  }
  label B_b:
  a[0]:byte = 4;
  label B_a:
  g_a = e + 16;
}

function f_kc(a:int_ptr, b:int_ptr):int {
  var g:int;
  var c:int = g_a - 16;
  g_a = c;
  var d:int = b[2];
  var e:int = b[0];
  var f:int = 1;
  f_jc(c, g = a[0], 1054416, 1);
  if (c[0]:ubyte != 4) goto B_c;
  if (d) goto B_d;
  f_jc(c, g, 1054417, 1);
  if (c[0]:ubyte != 4) goto B_b;
  f = 0;
  label B_d:
  b = d * 24;
  loop L_f {
    if (eqz(b)) goto B_e;
    if ((f & 255) == 1) goto B_g;
    f_jc(c, g, 1054418, 1);
    if (c[0]:ubyte == 4) goto B_g;
    c[1]:long = c[0]:long;
    d = f_cg(c + 8);
    goto B_a;
    label B_g:
    b = b + -24;
    f = 2;
    d = f_j(e, a);
    e = e + 24;
    if (eqz(d)) continue L_f;
    goto B_a;
  }
  label B_e:
  if (f & 255) goto B_h;
  d = 0;
  goto B_a;
  label B_h:
  f_jc(c, g, 1054417, 1);
  d = 0;
  if (c[0]:ubyte == 4) goto B_a;
  c[1]:long = c[0]:long;
  d = f_cg(c + 8);
  goto B_a;
  label B_c:
  c[1]:long = c[0]:long;
  d = f_cg(c + 8);
  goto B_a;
  label B_b:
  c[1]:long = c[0]:long;
  d = f_cg(c + 8);
  label B_a:
  g_a = c + 16;
  return d;
}

function f_lc(a:int, b:int) {
  var g:int_ptr;
  var f:int;
  var c:int_ptr = 0;
  if (b < 256) goto B_a;
  c = 31;
  if (b > 16777215) goto B_a;
  c = (b >> 6 - (c = clz(b >> 8)) & 1) - (c << 1) + 62;
  label B_a:
  a[4]:long@4 = 0L;
  a[7]:int = c;
  var d:int_ptr = (c << 2) + 1076808;
  var e:int_ptr = f_cl(a);
  f = 0[269135]:int;
  if (eqz(f & (g = 1 << c))) goto B_f;
  f = d[0];
  c = f_fh(c);
  if (f_ej(f_cl(f)) != b) goto B_e;
  c = f;
  goto B_d;
  label B_f:
  0[269135]:int = f | g;
  d[0] = a;
  a[6]:int = d;
  goto B_b;
  label B_e:
  d = b << c;
  loop L_g {
    g = f + (d >> 29 & 4) + 16;
    c = g[0];
    if (eqz(c)) goto B_c;
    d = d << 1;
    f = c;
    if (f_ej(f_cl(c)) != b) continue L_g;
  }
  label B_d:
  c = f_cl(c);
  d = c[2];
  d[3] = e;
  c[2] = e;
  e[3] = c;
  e[2] = d;
  a[6]:int = 0;
  return ;
  label B_c:
  g[0] = a;
  a[6]:int = f;
  label B_b:
  e[2] = e;
  e[3] = e;
}

function f_mc(a:int, b:int, c:int_ptr) {
  var f:int;
  var g:int;
  var d:int = g_a - 64;
  g_a = d;
  var e:int = 1;
  if (a[4]:ubyte) goto B_a;
  e = a[5]:ubyte;
  f = a[0]:int;
  g = f[0]:int;
  if (g & 4) goto B_e;
  if (e & 255) goto B_d;
  goto B_b;
  label B_e:
  if (e & 255) goto B_c;
  e = 1;
  if (
    call_indirect(f[6]:int, 1072614, 1, ((f + 28)[0]:int)[3]:int)) goto B_a;
  g = f[0]:int;
  goto B_c;
  label B_d:
  e = 1;
  if (
    eqz(call_indirect(f[6]:int, 1072601, 2, ((f + 28)[0]:int)[3]:int))) goto B_b;
  goto B_a;
  label B_c:
  e = 1;
  d[23]:byte = 1;
  (d + 52)[0]:int = 1072568;
  (d + 16)[0]:int = d + 23;
  d[6]:int = g;
  d[1]:long = f[6]:long@4;
  var h:long = f[2]:long@4;
  var i:long = f[4]:long@4;
  d[56]:byte = f[32]:ubyte;
  d[7]:int = f[1]:int;
  d[5]:long = i;
  d[4]:long = h;
  d[12]:int = d + 8;
  if (call_indirect(b, d + 24, c[3])) goto B_a;
  e = call_indirect(d[12]:int, 1072599, 2, (d[13]:int)[3]:int);
  goto B_a;
  label B_b:
  e = call_indirect(b, f, c[3]);
  label B_a:
  a[5]:byte = 1;
  a[4]:byte = e;
  g_a = d + 64;
}

function f_nc(a:int_ptr) {
  var c:int_ptr;
  var d:int_ptr;
  var e:int_ptr;
  var f:int_ptr;
  var b:int_ptr = a[6];
  if (f_pk(a) != a) goto B_c;
  e = (a + select_if(20, 16, d = (c = a + 20)[0]))[0]:int;
  if (e) goto B_b;
  d = 0;
  goto B_a;
  label B_c:
  e = f_qk(a);
  e[3] = f_cl(d = f_pk(a));
  d[2] = f_cl(e);
  goto B_a;
  label B_b:
  c = select_if(c, a + 16, d);
  loop L_d {
    f = c;
    d = e;
    c = d + 20;
    e = c[0];
    if (e) goto B_e;
    c = d + 16;
    e = d[4];
    label B_e:
    if (e) continue L_d;
  }
  f[0] = 0;
  label B_a:
  if (eqz(b)) goto B_f;
  e = (a[7] << 2) + 1076808;
  if (e[0] == a) goto B_h;
  (b + select_if(16, 20, b[4] == a))[0]:int = d;
  if (d) goto B_g;
  goto B_f;
  label B_h:
  e[0] = d;
  if (d) goto B_g;
  0[269135]:int = 0[269135]:int & -2 << a[7];
  return ;
  label B_g:
  d[6] = b;
  e = a[4];
  if (eqz(e)) goto B_i;
  d[4] = e;
  e[6] = d;
  label B_i:
  e = (a + 20)[0]:int;
  if (eqz(e)) goto B_f;
  (d + 20)[0]:int = e;
  e[6] = d;
  return ;
  label B_f:
}

function f_oc(a:{ a:int, b:int }, b:int) {
  var d:int;
  var g:int;
  var h:int;
  var i:int;
  var j:int;
  var f:int;
  var c:int;
  c = b[2]:int;
  if (c > (d = (b + 4)[0]:int)) goto B_a;
  if (c) goto B_c;
  c = 1;
  d = 0;
  goto B_b;
  label B_c:
  var e:int = c & 3;
  b = b[0]:int;
  if (c + -1 >= 3) goto B_e;
  d = 0;
  c = 1;
  goto B_d;
  label B_e:
  f = c & -4;
  c = 1;
  d = 0;
  loop L_f {
    d = select_if(
          0,
          select_if(1,
                    select_if(2,
                              select_if(3, d + 4, g = b[0]:ubyte == 10),
                              h = (b + 1)[0]:ubyte == 10),
                    i = (b + 2)[0]:ubyte == 10),
          j = (b + 3)[0]:ubyte == 10);
    c = c + g + h + i + j;
    b = b + 4;
    f = f + -4;
    if (f) continue L_f;
  }
  label B_d:
  if (eqz(e)) goto B_b;
  loop L_g {
    d = select_if(0, d + 1, f = b[0]:ubyte == 10);
    b = b + 1;
    c = c + f;
    e = e + -1;
    if (e) continue L_g;
  }
  label B_b:
  a.b = d;
  a.a = c;
  return ;
  label B_a:
  f_sj(c, d, 1053316);
  unreachable;
}

function f_pc(a:int_ptr, b:int_ptr):int {
  var c:int = g_a - 16;
  g_a = c;
  a = a[0];
  if (b[2] == 1) goto B_c;
  if (b[4] != 1) goto B_b;
  label B_c:
  c[3]:int = 0;
  if (a < 128) goto B_g;
  if (a < 2048) goto B_f;
  if (a >= 65536) goto B_e;
  c[14]:byte = (a & 63) | 128;
  c[12]:byte = a >> 12 | 224;
  c[13]:byte = (a >> 6 & 63) | 128;
  a = 3;
  goto B_d;
  label B_g:
  c[12]:byte = a;
  a = 1;
  goto B_d;
  label B_f:
  c[13]:byte = (a & 63) | 128;
  c[12]:byte = a >> 6 | 192;
  a = 2;
  goto B_d;
  label B_e:
  c[15]:byte = (a & 63) | 128;
  c[12]:byte = a >> 18 | 240;
  c[14]:byte = (a >> 6 & 63) | 128;
  c[13]:byte = (a >> 12 & 63) | 128;
  a = 4;
  label B_d:
  b = f_aa(b, c + 12, a);
  goto B_a;
  label B_b:
  b = call_indirect(b[6], a, ((b + 28)[0]:int)[4]:int);
  label B_a:
  g_a = c + 16;
  return b;
}

function f_qc(a:{ a:int, b:int }, b:int) {
  var e:int;
  var d:int;
  var g:int;
  var h:int;
  var i:int;
  var j:int;
  var f:int;
  var c:int = 1;
  d = b[2]:int + 1;
  d = select_if(d, e = (b + 4)[0]:int, e > d);
  if (d) goto B_b;
  d = 0;
  goto B_a;
  label B_b:
  e = d & 3;
  b = b[0]:int;
  if (d + -1 >= 3) goto B_d;
  d = 0;
  c = 1;
  goto B_c;
  label B_d:
  f = d & -4;
  c = 1;
  d = 0;
  loop L_e {
    d = select_if(
          0,
          select_if(1,
                    select_if(2,
                              select_if(3, d + 4, g = b[0]:ubyte == 10),
                              h = (b + 1)[0]:ubyte == 10),
                    i = (b + 2)[0]:ubyte == 10),
          j = (b + 3)[0]:ubyte == 10);
    c = c + g + h + i + j;
    b = b + 4;
    f = f + -4;
    if (f) continue L_e;
  }
  label B_c:
  if (eqz(e)) goto B_a;
  loop L_f {
    d = select_if(0, d + 1, f = b[0]:ubyte == 10);
    b = b + 1;
    c = c + f;
    e = e + -1;
    if (e) continue L_f;
  }
  label B_a:
  a.b = d;
  a.a = c;
}

function f_rc():int {
  var a:int;
  var h:int;
  a = 0[269242]:int;
  if (a) goto B_a;
  0[269246]:int = 4095;
  return 0;
  label B_a:
  var b:int_ptr = 1076960;
  var c:int = 0;
  var d:int = 0;
  loop L_b {
    var e:{ a:int, b:int, c:int } = a;
    a = e.c;
    var f:int = e.b;
    var g:int = e.a;
    if (eqz(f_gl(1076536, (e + 12)[0]:int >> 1))) goto B_d;
    if (f_gj(e)) goto B_d;
    h = g + f_nh(h = f_mk(g), 8) - h;
    var i:int = f_ej(h);
    var j:int = f_bl();
    var k:int = f_nh(j, 8);
    var l:int = f_nh(20, 8);
    var m:int = f_nh(16, 8);
    if (f_gi(h)) goto B_d;
    if (h + i < g + j + f - k + l + m) goto B_d;
    if (0[269236]:int == h) goto B_f;
    f_nc(h);
    goto B_e;
    label B_f:
    0[269234]:int = 0;
    0[269236]:int = 0;
    label B_e:
    if (f_fl(1076536, g, f)) goto B_g;
    f_lc(h, i);
    goto B_d;
    label B_g:
    0[269238]:int = 0[269238]:int - f;
    b[2] = a;
    c = f + c;
    goto B_c;
    label B_d:
    b = e;
    label B_c:
    d = d + 1;
    if (a) continue L_b;
  }
  0[269246]:int = select_if(d, 4095, d > 4095);
  return c;
}

function f_sc(a:{ a:int, b:int }, b:{ a:int, b:int, c:int }) {
  var e:int;
  var f:ushort_ptr;
  var d:int;
  var c:int = b.a;
  d = b.c;
  if (d >= (e = b.b)[201]:ushort) goto B_c;
  f = e;
  goto B_b;
  label B_c:
  loop L_d {
    f = e[66]:int;
    if (eqz(f)) goto B_a;
    c = c + 1;
    d = e[200]:ushort;
    e = f;
    if (d >= f[201]) continue L_d;
  }
  label B_b:
  var g:int = d + 1;
  if (c) goto B_f;
  e = f;
  goto B_e;
  label B_f:
  e = (f + (g << 2) + 408)[0]:int;
  g = 0;
  var h:int = c + -1;
  if (eqz(h)) goto B_e;
  var i:int = c + -2;
  c = h & 7;
  if (eqz(c)) goto B_g;
  loop L_h {
    h = h + -1;
    e = e[102]:int;
    c = c + -1;
    if (c) continue L_h;
  }
  label B_g:
  if (i < 7) goto B_e;
  loop L_i {
    e = 
      (((((((e[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int)[102]:int;
    h = h + -8;
    if (h) continue L_i;
  }
  label B_e:
  b.c = g;
  b.b = e;
  b.a = 0;
  a.b = f + d * 24;
  a.a = f + d * 12 + 268;
  return ;
  label B_a:
  f_rf(1053024, 43, 1053164);
  unreachable;
}

function f_tc(a:{ a:int, b:int }, b:int):int {
  br_table[B_v, B_u, B_t, B_s, B_r, B_q, B_p, B_o, B_n, B_m, B_l, B_k, B_j, B_i, B_h, B_g, B_f, B_e, B_d, B_c, B_b, B_a, ..B_v](
    a.a)
  label B_v:
  return f_ih(b, a.b, (a + 8)[0]:int);
  label B_u:
  return f_ob(a + 4, b);
  label B_t:
  return f_ih(b, 1055564, 24);
  label B_s:
  return f_ih(b, 1055537, 27);
  label B_r:
  return f_ih(b, 1055511, 26);
  label B_q:
  return f_ih(b, 1055486, 25);
  label B_p:
  return f_ih(b, 1055474, 12);
  label B_o:
  return f_ih(b, 1055455, 19);
  label B_n:
  return f_ih(b, 1055436, 19);
  label B_m:
  return f_ih(b, 1055422, 14);
  label B_l:
  return f_ih(b, 1055408, 14);
  label B_k:
  return f_ih(b, 1055394, 14);
  label B_j:
  return f_ih(b, 1055380, 14);
  label B_i:
  return f_ih(b, 1055361, 19);
  label B_h:
  return f_ih(b, 1055335, 26);
  label B_g:
  return f_ih(b, 1055273, 62);
  label B_f:
  return f_ih(b, 1055253, 20);
  label B_e:
  return f_ih(b, 1055217, 36);
  label B_d:
  return f_ih(b, 1055203, 14);
  label B_c:
  return f_ih(b, 1055184, 19);
  label B_b:
  return f_ih(b, 1055156, 28);
  label B_a:
  return f_ih(b, 1055132, 24);
}

function f_uc(a:{ a:int, b:int }) {
  var j:long;
  var g:int;
  var c:int;
  var d:long_ptr;
  var f:long_ptr;
  var b:int = a.a;
  if (eqz(b)) goto B_a;
  if ((a + 12)[0]:int) goto B_c;
  c = b + 1;
  goto B_b;
  label B_c:
  d = (a + 4)[0]:int;
  var e:int = d + (c = b + 1);
  f = d + 8;
  var i:long = (d[0] ^ -1L) & -9187201950435737472L;
  loop L_d {
    if (i == 0L) goto B_f;
    j = i;
    goto B_e;
    label B_f:
    loop L_g {
      if (f >= e) goto B_b;
      d = d + -160;
      j = f[0];
      g = f + 8;
      f = g;
      j = j & -9187201950435737472L;
      if (j == -9187201950435737472L) continue L_g;
    }
    j = j ^ -9187201950435737472L;
    f = g;
    label B_e:
    i = j + -1L & j;
    g = d + (0 - (i32_wrap_i64(ctz(j)) >> 3)) * 20;
    var h:int = (g + -8)[0]:int;
    if (eqz(h)) continue L_d;
    f_mi((g + -12)[0]:int, h, 1);
    continue L_d;
  }
  label B_b:
  d = b + (f = i32_wrap_i64(i64_extend_i32_u(c) * 20L) + 7 & -8) + 9;
  if (eqz(d)) goto B_a;
  f_mi(a.b - f, d, 8);
  label B_a:
}

function f_vc(a:int, b:int_ptr):int {
  var e:int;
  var f:int;
  var c:int = g_a - 16;
  g_a = c;
  var d:{ a:int, b:int } = a[0]:int;
  if (a[4]:ubyte == 1) goto B_e;
  f_jc(c + 8, e = d.a, 1054425, 2);
  if (c[8]:ubyte == 4) goto B_d;
  goto B_c;
  label B_e:
  f_jc(c + 8, e = d.a, 1054424, 1);
  if (c[8]:ubyte != 4) goto B_c;
  label B_d:
  f = d.b;
  if (eqz(f)) goto B_b;
  var g:int = (d + 12)[0]:int;
  var h:int = (d + 8)[0]:int;
  loop L_f {
    f_jc(c + 8, e, h, g);
    if (c[8]:ubyte != 4) goto B_c;
    f = f + -1;
    if (f) continue L_f;
    goto B_b;
  }
  label B_c:
  var i:long = c[1]:long;
  if ((i32_wrap_i64(i) & 255) == 4) goto B_b;
  c[1]:long = i;
  f = f_cg(c + 8);
  goto B_a;
  label B_b:
  a[4]:byte = 2;
  f_ta(c, d, c, b[0], b[2]);
  if (c[0]:ubyte == 4) goto B_g;
  c[1]:long = c[0]:long;
  f = f_cg(c + 8);
  goto B_a;
  label B_g:
  f = 0;
  label B_a:
  g_a = c + 16;
  return f;
}

function f_wc(a:{ a:int, b:int }, b:int) {
  var e:int;
  var c:int = g_a - 48;
  g_a = c;
  var d:long_ptr@4 = b + 4;
  if (b[1]:int) goto B_a;
  e = b[0]:int;
  var f:int_ptr = c + 8 + 8;
  f[0] = 0;
  c[1]:long = 1L;
  c[5]:int = c + 8;
  (c + 24 + 16)[0]:long = (e + 16)[0]:long@4;
  (c + 24 + 8)[0]:long = (e + 8)[0]:long@4;
  c[3]:long = e[0]:long@4;
  f_va(c + 20, 1067400, c + 24);
  (d + 8)[0]:int = f[0];
  d[0] = c[1]:long;
  label B_a:
  e = c + 24 + 8;
  e[0]:int = (d + 8)[0]:int;
  (b + 12)[0]:int = 0;
  var g:long = d[0];
  b[1]:long@4 = 1L;
  c[3]:long = g;
  b = f_wh(12, 4);
  if (b) goto B_b;
  f_mj(12, 4);
  unreachable;
  label B_b:
  b[0]:long@4 = c[3]:long;
  (b + 8)[0]:int = e[0]:int;
  a.b = 1068416;
  a.a = b;
  g_a = c + 48;
}

function f_xc(a:int, b:int_ptr):int {
  var d:int;
  var c:int = g_a - 16;
  g_a = c;
  a = a[0]:int;
  if (a[0]:ubyte) goto B_b;
  b = call_indirect(b[6], 1075598, 4, ((b + 28)[0]:int)[3]:int);
  goto B_a;
  label B_b:
  c[8]:byte = call_indirect(b[6], 1075594, 4, ((b + 28)[0]:int)[3]:int);
  c[0]:int = b;
  c[9]:byte = 0;
  c[1]:int = 0;
  b = 1;
  c[3]:int = a + 1;
  f_hc(c, c + 12, 1072616);
  a = c[8]:ubyte;
  d = c[1]:int;
  if (d) goto B_d;
  b = a;
  goto B_c;
  label B_d:
  if (a & 255) goto B_c;
  a = c[0]:int;
  if (d != 1) goto B_e;
  if (eqz(c[9]:ubyte & 255)) goto B_e;
  if (a[0]:ubyte & 4) goto B_e;
  b = 1;
  if (
    call_indirect(a[6]:int, 1072612, 1, ((a + 28)[0]:int)[3]:int)) goto B_c;
  label B_e:
  b = call_indirect(a[6]:int, 1072613, 1, ((a + 28)[0]:int)[3]:int);
  label B_c:
  b = (b & 255) != 0;
  label B_a:
  g_a = c + 16;
  return b;
}

function f_yc(a:int_ptr, b:int):int {
  var c:int = g_a - 16;
  g_a = c;
  a = a[0];
  c[3]:int = 0;
  if (b < 128) goto B_d;
  if (b < 2048) goto B_c;
  if (b >= 65536) goto B_b;
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  goto B_a;
  label B_d:
  c[12]:byte = b;
  b = 1;
  goto B_a;
  label B_c:
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_a;
  label B_b:
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  label B_a:
  b = f_ia(a, c + 12, b);
  g_a = c + 16;
  return b;
}

function f_zc(a:int, b:int):int {
  var c:int = g_a - 16;
  g_a = c;
  c[3]:int = 0;
  if (b < 128) goto B_d;
  if (b < 2048) goto B_c;
  if (b >= 65536) goto B_b;
  c[14]:byte = (b & 63) | 128;
  c[12]:byte = b >> 12 | 224;
  c[13]:byte = (b >> 6 & 63) | 128;
  b = 3;
  goto B_a;
  label B_d:
  c[12]:byte = b;
  b = 1;
  goto B_a;
  label B_c:
  c[13]:byte = (b & 63) | 128;
  c[12]:byte = b >> 6 | 192;
  b = 2;
  goto B_a;
  label B_b:
  c[15]:byte = (b & 63) | 128;
  c[12]:byte = b >> 18 | 240;
  c[14]:byte = (b >> 6 & 63) | 128;
  c[13]:byte = (b >> 12 & 63) | 128;
  b = 4;
  label B_a:
  b = f_ia(a, c + 12, b);
  g_a = c + 16;
  return b;
}

function f_ad(a:int_ptr, b:int):int {
  var c:int = g_a - 112;
  g_a = c;
  c[16]:int = 0;
  c[7]:long = 1L;
  a = a[0];
  f_fg(c + 72, c + 56, 1054720);
  if (f_tc(a, c + 72)) goto B_a;
  (c + 32 + 20)[0]:int = 29;
  (c + 32 + 12)[0]:int = 29;
  (c + 8 + 20)[0]:int = 3;
  c[9]:int = 31;
  c[3]:long@4 = 4L;
  c[2]:int = 1055660;
  c[12]:int = a + 16;
  c[10]:int = a + 12;
  c[8]:int = c + 56;
  c[6]:int = c + 32;
  a = f_pe(b, c + 8);
  b = c[15]:int;
  if (eqz(b)) goto B_b;
  f_mi(c[14]:int, b, 1);
  label B_b:
  g_a = c + 112;
  return a;
  label B_a:
  f_ee(1054744, 55, c + 32, 1055036, 1054876);
  return unreachable;
}

function f_bd(a:int, b:int_ptr, c:int_ptr, d:int, e:int) {
  var h:int;
  var f:{ a:long, b:long, c:int, d:int, e:byte } = g_a - 32;
  g_a = f;
  var g:int = 1;
  0[269127]:int = (h = 0[269127]:int) + 1;
  if (eqz(0[1076988]:ubyte)) goto B_b;
  g = 0[269248]:int + 1;
  goto B_a;
  label B_b:
  0[1076988]:byte = 1;
  label B_a:
  0[269248]:int = g;
  if (h < 0) goto B_d;
  if (g > 2) goto B_d;
  f.e = e;
  f.d = d;
  f.c = c;
  h = 0[269124]:int;
  if (h <= -1) goto B_d;
  0[269124]:int = (h = h + 1);
  c = 0[269126]:int;
  if (eqz(c)) goto B_e;
  h = 0[269125]:int;
  call_indirect(f, a, b[4]);
  f.b = f.a;
  call_indirect(h, f + 8, c[5]);
  h = 0[269124]:int;
  label B_e:
  0[269124]:int = h + -1;
  if (g > 1) goto B_d;
  if (e) goto B_c;
  label B_d:
  unreachable;
  label B_c:
  f_og(a, b);
  unreachable;
}

function f_cd(a:ubyte_ptr, b:int_ptr):int {
  var c:int = g_a - 48;
  g_a = c;
  if (eqz(a[4])) goto B_b;
  c[7]:byte = (a + 5)[0]:ubyte;
  (c + 20)[0]:int = 29;
  c[4]:int = a;
  c[3]:int = 75;
  a = (b + 28)[0]:int;
  c[2]:int = c + 7;
  b = b[6];
  (c + 44)[0]:int = 2;
  c[7]:long@4 = 2L;
  c[6]:int = 1073264;
  c[10]:int = c + 8;
  a = f_va(b, a, c + 24);
  goto B_a;
  label B_b:
  c[3]:int = 29;
  c[2]:int = a;
  a = (b + 28)[0]:int;
  b = b[6];
  (c + 44)[0]:int = 1;
  c[7]:long@4 = 1L;
  c[6]:int = 1073212;
  c[10]:int = c + 8;
  a = f_va(b, a, c + 24);
  label B_a:
  g_a = c + 48;
  return a;
}

function f_dd(a:int, b:int):int {
  var c:int = g_a - 16;
  g_a = c;
  var d:int = 
    call_indirect(b[6]:int, 1075607, 9, ((b + 28)[0]:int)[3]:int);
  c[5]:byte = 0;
  c[4]:byte = d;
  c[0]:int = b;
  c[3]:int = a;
  b = f_rb(c, 1075616, 11, c + 12, 1072376);
  c[3]:int = a + 4;
  f_rb(b, 1075627, 9, c + 12, 1075636);
  b = c[4]:ubyte;
  if (eqz(c[5]:ubyte)) goto B_a;
  a = b & 255;
  b = 1;
  if (a) goto B_a;
  b = c[0]:int;
  if (b[0]:ubyte & 4) goto B_b;
  b = call_indirect(b[6]:int, 1072607, 2, ((b + 28)[0]:int)[3]:int);
  goto B_a;
  label B_b:
  b = call_indirect(b[6]:int, 1072606, 1, ((b + 28)[0]:int)[3]:int);
  label B_a:
  g_a = c + 16;
  return (b & 255) != 0;
}

function f_ed(a:{ a:byte, b:byte }, b:ubyte_ptr, c:int) {
  var g:long;
  if (c) goto B_a;
  a.b = 0;
  a.a = 1;
  return ;
  label B_a:
  br_table[B_d, B_c, B_e, ..B_c](b[0] + -43)
  label B_e:
  if (c != 1) goto B_c;
  goto B_b;
  label B_d:
  c = c + -1;
  if (eqz(c)) goto B_b;
  b = b + 1;
  label B_c:
  var d:int = 0;
  loop L_g {
    if (eqz(c)) goto B_f;
    var e:int = b[0] + -48;
    if (e >= 10) goto B_b;
    g = i64_extend_i32_u(d) * 10L;
    if (eqz(i32_wrap_i64(g >> 32L))) goto B_h;
    a.b = 2;
    a.a = 1;
    return ;
    label B_h:
    b = b + 1;
    c = c + -1;
    var f:int = i32_wrap_i64(g);
    d = f + e;
    if (d >= f) continue L_g;
  }
  a.b = 2;
  a.a = 1;
  return ;
  label B_f:
  (a + 4)[0]:int = d;
  a.a = 0;
  return ;
  label B_b:
  a.b = 1;
  a.a = 1;
}

function f_fd(a:{ a:int, b:int }, b:int, c:{ a:byte, b:byte, c:byte, d:byte }) {
  if (b < 128) goto B_d;
  if (b < 2048) goto B_c;
  if (b >= 65536) goto B_b;
  c.c = (b & 63) | 128;
  c.a = b >> 12 | 224;
  c.b = (b >> 6 & 63) | 128;
  b = 3;
  goto B_a;
  label B_d:
  c.a = b;
  b = 1;
  goto B_a;
  label B_c:
  c.b = (b & 63) | 128;
  c.a = b >> 6 | 192;
  b = 2;
  goto B_a;
  label B_b:
  c.d = (b & 63) | 128;
  c.a = b >> 18 | 240;
  c.c = (b >> 6 & 63) | 128;
  c.b = (b >> 12 & 63) | 128;
  b = 4;
  label B_a:
  a.b = b;
  a.a = c;
}

function f_gd(a:int_ptr, b:int) {
  var c:int_ptr = g_a - 32;
  g_a = c;
  var d:int = b + 1;
  if (d < b) goto B_a;
  var e:int = (a + 4)[0]:int;
  b = e << 1;
  b = select_if(b, d, b > d);
  b = select_if(b, 8, b > 8);
  if (eqz(e)) goto B_c;
  (c + 16 + 8)[0]:int = 1;
  c[5] = e;
  c[4] = a[0];
  goto B_b;
  label B_c:
  c[4] = 0;
  label B_b:
  f_sd(c, b, 1, c + 16);
  if (eqz(c[0])) goto B_d;
  a = (c + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mj(c[1], a);
  unreachable;
  label B_d:
  d = c[1];
  (a + 4)[0]:int = b;
  a[0] = d;
  g_a = c + 32;
  return ;
  label B_a:
  f_zf();
  unreachable;
}

function f_hd(a:int_ptr, b:int, c:int) {
  var d:int_ptr = g_a - 32;
  g_a = d;
  c = b + c;
  if (c < b) goto B_a;
  var e:int = (a + 4)[0]:int;
  b = e << 1;
  b = select_if(b, c, b > c);
  b = select_if(b, 8, b > 8);
  if (eqz(e)) goto B_c;
  (d + 16 + 8)[0]:int = 1;
  d[5] = e;
  d[4] = a[0];
  goto B_b;
  label B_c:
  d[4] = 0;
  label B_b:
  f_sd(d, b, 1, d + 16);
  if (eqz(d[0])) goto B_d;
  a = (d + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mj(d[1], a);
  unreachable;
  label B_d:
  c = d[1];
  (a + 4)[0]:int = b;
  a[0] = c;
  g_a = d + 32;
  return ;
  label B_a:
  f_zf();
  unreachable;
}

function f_id(a:int_ptr, b:int) {
  var c:int_ptr = g_a - 32;
  g_a = c;
  var d:int = b + 1;
  if (d < b) goto B_a;
  var e:int = (a + 4)[0]:int;
  b = e << 1;
  b = select_if(b, d, b > d);
  b = select_if(b, 8, b > 8);
  if (eqz(e)) goto B_c;
  (c + 16 + 8)[0]:int = 1;
  c[5] = e;
  c[4] = a[0];
  goto B_b;
  label B_c:
  c[4] = 0;
  label B_b:
  f_td(c, b, 1, c + 16);
  if (eqz(c[0])) goto B_d;
  a = (c + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mj(c[1], a);
  unreachable;
  label B_d:
  d = c[1];
  (a + 4)[0]:int = b;
  a[0] = d;
  g_a = c + 32;
  return ;
  label B_a:
  f_zf();
  unreachable;
}

function f_jd(a:int_ptr, b:int, c:int) {
  var d:int_ptr = g_a - 32;
  g_a = d;
  c = b + c;
  if (c < b) goto B_a;
  var e:int = (a + 4)[0]:int;
  b = e << 1;
  b = select_if(b, c, b > c);
  b = select_if(b, 8, b > 8);
  if (eqz(e)) goto B_c;
  (d + 16 + 8)[0]:int = 1;
  d[5] = e;
  d[4] = a[0];
  goto B_b;
  label B_c:
  d[4] = 0;
  label B_b:
  f_td(d, b, 1, d + 16);
  if (eqz(d[0])) goto B_d;
  a = (d + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mj(d[1], a);
  unreachable;
  label B_d:
  c = d[1];
  (a + 4)[0]:int = b;
  a[0] = c;
  g_a = d + 32;
  return ;
  label B_a:
  f_zf();
  unreachable;
}

function f_kd(a:int_ptr, b:int, c:int) {
  var d:int_ptr = g_a - 32;
  g_a = d;
  c = b + c;
  if (c < b) goto B_a;
  var e:int = (a + 4)[0]:int;
  b = e << 1;
  b = select_if(b, c, b > c);
  b = select_if(b, 8, b > 8);
  if (eqz(e)) goto B_c;
  (d + 16 + 8)[0]:int = 1;
  d[5] = e;
  d[4] = a[0];
  goto B_b;
  label B_c:
  d[4] = 0;
  label B_b:
  f_rd(d, b, 1, d + 16);
  if (eqz(d[0])) goto B_d;
  a = (d + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mj(d[1], a);
  unreachable;
  label B_d:
  c = d[1];
  (a + 4)[0]:int = b;
  a[0] = c;
  g_a = d + 32;
  return ;
  label B_a:
  f_zf();
  unreachable;
}

function f_ld(a:int_ptr, b:int) {
  var c:int_ptr = g_a - 32;
  g_a = c;
  var d:int = b + 1;
  if (d < b) goto B_a;
  var e:int = (a + 4)[0]:int;
  b = e << 1;
  b = select_if(b, d, b > d);
  b = select_if(b, 8, b > 8);
  if (eqz(e)) goto B_c;
  (c + 16 + 8)[0]:int = 1;
  c[5] = e;
  c[4] = a[0];
  goto B_b;
  label B_c:
  c[4] = 0;
  label B_b:
  f_rd(c, b, 1, c + 16);
  if (eqz(c[0])) goto B_d;
  a = (c + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mj(c[1], a);
  unreachable;
  label B_d:
  d = c[1];
  (a + 4)[0]:int = b;
  a[0] = d;
  g_a = c + 32;
  return ;
  label B_a:
  f_zf();
  unreachable;
}

function f_md(a:int, b:int_ptr, c:int):int {
  var d:int = g_a - 16;
  g_a = d;
  var e:int_ptr = a[0]:int;
  if (a[4]:ubyte == 1) goto B_b;
  f_jc(d, e[0], 1054418, 1);
  if (d[0]:ubyte == 4) goto B_b;
  d[1]:long = d[0]:long;
  e = f_cg(d + 8);
  goto B_a;
  label B_b:
  a[4]:byte = 2;
  f_sa(d, e, d, b[0], b[2]);
  if (d[0]:ubyte == 4) goto B_c;
  d[1]:long = d[0]:long;
  e = f_cg(d + 8);
  goto B_a;
  label B_c:
  f_jc(d, e[0], 1054421, 1);
  if (d[0]:ubyte == 4) goto B_d;
  d[1]:long = d[0]:long;
  e = f_cg(d + 8);
  goto B_a;
  label B_d:
  e = f_j(c, e);
  label B_a:
  g_a = d + 16;
  return e;
}

function f_nd(a:int, b:int, c:int):int {
  var e:int;
  var d:byte_ptr;
  var f:int_ptr;
  if (c > 15) goto B_b;
  d = a;
  goto B_a;
  label B_b:
  f = a + (e = 0 - a & 3);
  if (eqz(e)) goto B_c;
  d = a;
  loop L_d {
    d[0] = b;
    d = d + 1;
    if (d < f) continue L_d;
  }
  label B_c:
  d = f + (c = (e = c - e) & -4);
  if (c < 1) goto B_e;
  c = (b & 255) * 16843009;
  loop L_f {
    f[0] = c;
    f = f + 4;
    if (f < d) continue L_f;
  }
  label B_e:
  c = e & 3;
  label B_a:
  if (eqz(c)) goto B_g;
  f = d + c;
  loop L_h {
    d[0] = b;
    d = d + 1;
    if (d < f) continue L_h;
  }
  label B_g:
  return a;
}

function f_od(a:ubyte_ptr, b:int, c:int):int {
  var d:int = g_a - 48;
  g_a = d;
  d[1]:int = c;
  d[0]:int = b;
  if (a[0] != 7) goto B_b;
  (d + 28)[0]:int = 1;
  d[3]:long@4 = 1L;
  d[2]:int = 1055768;
  d[9]:int = 32;
  d[6]:int = d + 32;
  d[8]:int = d;
  a = f_ff(d + 8);
  goto B_a;
  label B_b:
  (d + 44)[0]:int = 32;
  (d + 28)[0]:int = 2;
  d[3]:long@4 = 2L;
  d[2]:int = 1055720;
  d[9]:int = 33;
  d[8]:int = a;
  d[6]:int = d + 32;
  d[10]:int = d;
  a = f_ff(d + 8);
  label B_a:
  g_a = d + 48;
  return a;
}

function f_pd(a:{ a:int, b:int }, b:int) {
  var c:int = g_a - 48;
  g_a = c;
  var d:long_ptr@4 = b + 4;
  if (b[1]:int) goto B_a;
  b = b[0]:int;
  var e:int_ptr = c + 8 + 8;
  e[0] = 0;
  c[1]:long = 1L;
  c[5]:int = c + 8;
  (c + 24 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 24 + 8)[0]:long = (b + 8)[0]:long@4;
  c[3]:long = b[0]:long@4;
  f_va(c + 20, 1067400, c + 24);
  (d + 8)[0]:int = e[0];
  d[0] = c[1]:long;
  label B_a:
  a.b = 1068416;
  a.a = d;
  g_a = c + 48;
}

function f_qd(a:{ a:int, b:int }, b:int_ptr, c:int, d:int, e:int_ptr) {
  var g:int;
  var f:int_ptr = g_a - 16;
  g_a = f;
  if (d) goto B_d;
  if (e) goto B_c;
  label B_d:
  e = b + 8;
  d = e[0];
  if (d >= (g = (b + 4)[0]:int)) goto B_b;
  b = b[0];
  loop L_e {
    if (((b + d)[0]:ubyte + -48 & 255) >= 10) goto B_b;
    e[0] = (d = d + 1);
    if (g != d) continue L_e;
    goto B_b;
  }
  label B_c:
  f[0] = 13;
  a.b = f_ef(b, f);
  d = 1;
  goto B_a;
  label B_b:
  (a + 8)[0]:double = select_if(0.0, -0.0, c);
  d = 0;
  label B_a:
  a.a = d;
  g_a = f + 16;
}

function f_rd(a:{ a:int, b:int }, b:int, c:int, d:{ a:int, b:int }) {
  var f:int;
  var e:int;
  if (eqz(c)) goto B_i;
  e = 1;
  if (b < 0) goto B_h;
  f = d.a;
  if (eqz(f)) goto B_g;
  d = d.b;
  if (d) goto B_d;
  if (b) goto B_f;
  d = c;
  goto B_e;
  label B_i:
  a.b = b;
  e = 1;
  label B_h:
  b = 0;
  goto B_a;
  label B_g:
  if (b) goto B_f;
  d = c;
  goto B_e;
  label B_f:
  d = f_wh(b, c);
  label B_e:
  if (eqz(d)) goto B_c;
  goto B_b;
  label B_d:
  d = f_ph(f, d, c, b);
  if (d) goto B_b;
  label B_c:
  a.b = b;
  b = c;
  goto B_a;
  label B_b:
  a.b = d;
  e = 0;
  label B_a:
  a.a = e;
  (a + 8)[0]:int = b;
}

function f_sd(a:{ a:int, b:int }, b:int, c:int, d:{ a:int, b:int }) {
  var f:int;
  var e:int;
  if (eqz(c)) goto B_i;
  e = 1;
  if (b < 0) goto B_h;
  f = d.a;
  if (eqz(f)) goto B_f;
  d = d.b;
  if (d) goto B_g;
  if (b) goto B_e;
  goto B_c;
  label B_i:
  a.b = b;
  e = 1;
  label B_h:
  b = 0;
  goto B_a;
  label B_g:
  d = f_ph(f, d, c, b);
  if (eqz(d)) goto B_d;
  goto B_b;
  label B_f:
  if (eqz(b)) goto B_c;
  label B_e:
  d = f_wh(b, c);
  if (d) goto B_b;
  label B_d:
  a.b = b;
  b = c;
  goto B_a;
  label B_c:
  d = c;
  label B_b:
  a.b = d;
  e = 0;
  label B_a:
  a.a = e;
  (a + 8)[0]:int = b;
}

function f_td(a:{ a:int, b:int }, b:int, c:int, d:{ a:int, b:int }) {
  var f:int;
  var e:int;
  if (eqz(c)) goto B_i;
  e = 1;
  if (b < 0) goto B_h;
  f = d.a;
  if (eqz(f)) goto B_f;
  d = d.b;
  if (d) goto B_g;
  if (b) goto B_e;
  goto B_c;
  label B_i:
  a.b = b;
  e = 1;
  label B_h:
  b = 0;
  goto B_a;
  label B_g:
  d = f_ph(f, d, c, b);
  if (eqz(d)) goto B_d;
  goto B_b;
  label B_f:
  if (eqz(b)) goto B_c;
  label B_e:
  d = f_wh(b, c);
  if (d) goto B_b;
  label B_d:
  a.b = b;
  b = c;
  goto B_a;
  label B_c:
  d = c;
  label B_b:
  a.b = d;
  e = 0;
  label B_a:
  a.a = e;
  (a + 8)[0]:int = b;
}

function f_ud(a:ubyte_ptr, b:int):int {
  var e:int;
  var c:int = g_a - 128;
  g_a = c;
  var d:int = a[0];
  a = 0;
  loop L_a {
    (c + a + 127)[0]:byte = select_if(48, 87, (e = d & 15) < 10) + e;
    a = a + -1;
    e = d & 255;
    d = e >> 4;
    if (e > 15) continue L_a;
  }
  d = a + 128;
  if (d < 129) goto B_b;
  f_rj(d, 128, a);
  unreachable;
  label B_b:
  a = f_ca(b, 1, 1072633, 2, c + a + 128, 0 - a);
  g_a = c + 128;
  return a;
}

function f_vd(a:ubyte_ptr, b:int):int {
  var e:int;
  var c:int = g_a - 128;
  g_a = c;
  var d:int = a[0];
  a = 0;
  loop L_a {
    (c + a + 127)[0]:byte = select_if(48, 55, (e = d & 15) < 10) + e;
    a = a + -1;
    e = d & 255;
    d = e >> 4;
    if (e > 15) continue L_a;
  }
  d = a + 128;
  if (d < 129) goto B_b;
  f_rj(d, 128, a);
  unreachable;
  label B_b:
  a = f_ca(b, 1, 1072633, 2, c + a + 128, 0 - a);
  g_a = c + 128;
  return a;
}

function f_wd(a:int_ptr) {
  var c:int;
  var b:int = a[2];
  if (eqz(b)) goto B_a;
  b = b * 24;
  a = a[0] + 4;
  loop L_b {
    br_table[B_c, B_c, B_c, B_e, B_d, ..B_f]((a + -4)[0]:ubyte)
    label B_f:
    f_oa(a);
    goto B_c;
    label B_e:
    c = (a + 4)[0]:int;
    if (eqz(c)) goto B_c;
    f_mi(a[0], c, 1);
    goto B_c;
    label B_d:
    f_wd(a);
    c = (a + 4)[0]:int;
    if (eqz(c)) goto B_c;
    c = i32_wrap_i64(i64_extend_i32_u(c) * 24L);
    if (eqz(c)) goto B_c;
    f_mi(a[0], c, 8);
    label B_c:
    a = a + 24;
    b = b + -24;
    if (b) continue L_b;
  }
  label B_a:
}

function f_xd(a:int_ptr, b:int):int {
  var c:int = g_a - 48;
  g_a = c;
  a = a[0];
  if (a[3]) goto B_b;
  a = f_tc(a, b);
  goto B_a;
  label B_b:
  (c + 24 + 20)[0]:int = 29;
  (c + 24 + 12)[0]:int = 29;
  (c + 20)[0]:int = 3;
  c[1]:long@4 = 3L;
  c[0]:int = 1055608;
  c[8]:int = a + 12;
  c[7]:int = 30;
  c[6]:int = a;
  c[10]:int = a + 16;
  c[4]:int = c + 24;
  a = f_pe(b, c);
  label B_a:
  g_a = c + 48;
  return a;
}

function f_yd(a:{ a:int, b:int }, b:int, c:int) {
  var d:{ a:int, b:int } = g_a - 16;
  g_a = d;
  var e:int = (a + 20)[0]:int;
  br_table[B_d, B_c, ..B_a]((a + 4)[0]:int)
  label B_d:
  if (e) goto B_a;
  a = 1067424;
  e = 0;
  goto B_b;
  label B_c:
  if (e) goto B_a;
  a = a.a;
  e = a.b;
  a = a.a;
  label B_b:
  d.b = e;
  d.a = a;
  f_bd(d, 1068468, f_rk(b), c, f_tk(b));
  unreachable;
  label B_a:
  d.b = 0;
  d.a = a;
  f_bd(d, 1068448, f_rk(b), c, f_tk(b));
  unreachable;
}

function f_zd(a:int_ptr) {
  var b:int;
  var c:{ a:int, b:int }
  b = a[0];
  br_table[B_c, B_b, ..B_a](b[0]:int);
  label B_c:
  c = (b + 8)[0]:int;
  if (eqz(c)) goto B_a;
  f_mi(b[1]:int, c, 1);
  goto B_a;
  label B_b:
  if (b[4]:ubyte != 3) goto B_a;
  c = (b + 8)[0]:int;
  call_indirect(c.a, c.b[0]:int);
  var d:int_ptr = c.b;
  var e:int = d[1];
  if (eqz(e)) goto B_d;
  f_mi(c.a, e, d[2]);
  label B_d:
  f_mi(b[2]:int, 12, 4);
  label B_a:
  f_mi(a[0], 20, 4);
}

function f_ae(a:int_ptr, b:int) {
  var c:long_ptr@1 = g_a - 16;
  g_a = c;
  (c + 12)[0]:int@1 = (b + 8)[0]:int@1;
  a[0] = 0;
  (a + 8)[0]:byte = 5;
  c[4] = b[0]:long@1;
  (a + 9)[0]:long@1 = c[1];
  (a + 16)[0]:long@1 = (c + 8)[0]:long@1;
  a = b[3]:int;
  if (eqz(a)) goto B_a;
  b = (b + 16)[0]:int;
  if (eqz(b)) goto B_a;
  f_mi(a, b, 1);
  label B_a:
  g_a = c + 16;
}

function f_be(a:int_ptr) {
  var b:int;
  var c:{ a:int, b:int }
  b = a[0];
  br_table[B_c, B_b, ..B_a](b[0]:int);
  label B_c:
  c = (b + 8)[0]:int;
  if (eqz(c)) goto B_a;
  f_mi(b[1]:int, c, 1);
  goto B_a;
  label B_b:
  if (b[4]:ubyte != 3) goto B_a;
  c = (b + 8)[0]:int;
  call_indirect(c.a, c.b[0]:int);
  var d:int_ptr = c.b;
  var e:int = d[1];
  if (eqz(e)) goto B_d;
  f_mi(c.a, e, d[2]);
  label B_d:
  f_mi(b[2]:int, 12, 4);
  label B_a:
  f_mi(a[0], 20, 4);
}

function f_ce(a:{ a:long, b:long }, b:long, c:long, d:long, e:long) {
  var f:long;
  var g:long;
  var h:long;
  var i:long;
  var j:long;
  var k:long;
  a.a =
    (g = (h = (f = d & 4294967295L) * (g = b & 4294967295L)) +
         ((f = (j = f * (i = b >> 32L)) + (k = d >> 32L) * g) << 32L));
  a.b =
    k * i + (i64_extend_i32_u(f < j) << 32L | f >> 32L) +
    i64_extend_i32_u(g < h) +
    e * b + d * c;
}

function f_de(a:int, b:int) {
  var c:int_ptr;
  var d:int;
  if ((a + 4)[0]:int - (d = (c = a + 8)[0]) > 2) goto B_a;
  f_jd(a, d, 3);
  d = c[0];
  label B_a:
  c[0] = d + 3;
  a = a[0]:int + d;
  a[2]:byte = (b & 63) | 128;
  a[0]:byte = (d = b & 65535) >> 12 | 224;
  a[1]:byte = (d >> 6 & 63) | 128;
}

function f_ee(a:int, b:int, c:int, d:int, e:int) {
  var f:int = g_a - 64;
  g_a = f;
  f[3]:int = b;
  f[2]:int = a;
  f[5]:int = d;
  f[4]:int = c;
  (f + 44)[0]:int = 2;
  (f + 60)[0]:int = 73;
  f[7]:long@4 = 2L;
  f[6]:int = 1072552;
  f[13]:int = 69;
  f[10]:int = f + 48;
  f[14]:int = f + 16;
  f[12]:int = f + 8;
  f_ag(f + 24, e);
  unreachable;
}

function f_fe(a:int):int {
  var b:int = a[4]:ubyte;
  if (eqz(a[5]:ubyte)) goto B_a;
  var c:int = b & 255;
  b = 1;
  if (c) goto B_b;
  b = a[0]:int;
  if (b[0]:ubyte & 4) goto B_c;
  b = call_indirect(b[6]:int, 1072607, 2, ((b + 28)[0]:int)[3]:int);
  goto B_b;
  label B_c:
  b = call_indirect(b[6]:int, 1072606, 1, ((b + 28)[0]:int)[3]:int);
  label B_b:
  a[4]:byte = b;
  label B_a:
  return (b & 255) != 0;
}

function f_ge(a:int_ptr, b:int) {
  var c:int;
  a = f_kk(a, c = f_nh(c = f_mk(a), 8) - c);
  0[269235]:int = (b = b - c);
  0[269237]:int = a;
  a[1] = b | 1;
  c = f_bl();
  var d:int = f_nh(c, 8);
  var e:int = f_nh(20, 8);
  var f:int = f_nh(16, 8);
  f_kk(a, b)[1]:int = f + e + d - c;
  0[269244]:int = 2097152;
}

function f_he(a:int, b:int_ptr):int {
  var c:int = g_a - 32;
  g_a = c;
  var d:int = 1;
  if (f_fc(a, b)) goto B_a;
  var e:int = (b + 28)[0]:int;
  var f:int = b[6];
  (c + 28)[0]:int = 0;
  c[6]:int = 1072168;
  c[3]:long@4 = 1L;
  c[2]:int = 1072232;
  if (f_va(f, e, c + 8)) goto B_a;
  d = f_fc(a + 4, b);
  label B_a:
  g_a = c + 32;
  return d;
}

function f_ie(a:int, b:int, c:int):int {
  var e:long;
  var d:int = g_a - 16;
  g_a = d;
  br_table[B_d, B_c, B_b, ..B_d](a[0]:int)
  label B_d:
  var f:double = a[1]:double;
  d[0]:byte = 3;
  d[1]:double = f;
  goto B_a;
  label B_c:
  e = a[1]:long;
  d[0]:byte = 1;
  d[1]:long = e;
  goto B_a;
  label B_b:
  e = a[1]:long;
  d[0]:byte = 2;
  d[1]:long = e;
  label B_a:
  a = f_od(d, b, c);
  g_a = d + 16;
  return a;
}

function f_je(a:int_ptr, b:int):int {
  var c:int_ptr = g_a - 16;
  g_a = c;
  a = a[0];
  var d:int = (a + 8)[0]:int;
  a = a[0];
  f_hg(c, b);
  if (eqz(d)) goto B_a;
  loop L_b {
    c[3] = a;
    f_wi(c, c + 12, 1069152);
    a = a + 1;
    d = d + -1;
    if (d) continue L_b;
  }
  label B_a:
  a = f_kg(c);
  g_a = c + 16;
  return a;
}

function f_ke(a:int, b:int) {
  var c:int = g_a - 48;
  g_a = c;
  c[1]:int = b;
  c[0]:int = a;
  (c + 28)[0]:int = 2;
  (c + 44)[0]:int = 29;
  c[3]:long@4 = 2L;
  c[2]:int = 1073136;
  c[9]:int = 29;
  c[6]:int = c + 32;
  c[10]:int = c + 4;
  c[8]:int = c;
  f_ag(c + 8, 1073152);
  unreachable;
}

function f_le(a:int, b:int) {
  var c:int = g_a - 48;
  g_a = c;
  c[1]:int = b;
  c[0]:int = a;
  (c + 28)[0]:int = 2;
  (c + 44)[0]:int = 29;
  c[3]:long@4 = 2L;
  c[2]:int = 1073068;
  c[9]:int = 29;
  c[6]:int = c + 32;
  c[10]:int = c + 4;
  c[8]:int = c;
  f_ag(c + 8, 1073084);
  unreachable;
}

function f_me(a:int, b:int) {
  var c:int = g_a - 48;
  g_a = c;
  c[1]:int = b;
  c[0]:int = a;
  (c + 28)[0]:int = 2;
  (c + 44)[0]:int = 29;
  c[3]:long@4 = 2L;
  c[2]:int = 1072988;
  c[9]:int = 29;
  c[6]:int = c + 32;
  c[10]:int = c + 4;
  c[8]:int = c;
  f_ag(c + 8, 1073036);
  unreachable;
}

function f_ne(a:int, b:int, c:int) {
  var d:int = g_a - 48;
  g_a = d;
  d[1]:int = b;
  d[0]:int = a;
  (d + 28)[0]:int = 2;
  (d + 44)[0]:int = 29;
  d[3]:long@4 = 2L;
  d[2]:int = 1072360;
  d[9]:int = 29;
  d[6]:int = d + 32;
  d[10]:int = d;
  d[8]:int = d + 4;
  f_ag(d + 8, c);
  unreachable;
}

function f_oe(a:long_ptr@4, b:int_ptr):int {
  var c:long_ptr = g_a - 32;
  g_a = c;
  var d:int = (b + 28)[0]:int;
  b = b[6];
  (c + 8 + 16)[0]:long = (a + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (a + 8)[0]:long@4;
  c[1] = a[0];
  a = f_va(b, d, c + 8);
  g_a = c + 32;
  return a;
}

function f_pe(a:int_ptr, b:long_ptr@4):int {
  var c:long_ptr = g_a - 32;
  g_a = c;
  var d:int = (a + 28)[0]:int;
  a = a[6];
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1] = b[0];
  b = f_va(a, d, c + 8);
  g_a = c + 32;
  return b;
}

function f_qe(a:int, b:int, c:long_ptr@4) {
  var d:{ a:int, b:int, c:long } = g_a - 32;
  g_a = d;
  d.b = b;
  d.a = a;
  (d + 8 + 16)[0]:long = (c + 16)[0]:long@4;
  (d + 8 + 8)[0]:long = (c + 8)[0]:long@4;
  d.c = c[0];
  f_pb(0, d, 1072392, d + 4, 1072392, d + 8, 1069380);
  unreachable;
}

function f_re(a:int_ptr, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a[0];
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1049128, c + 8);
  g_a = c + 32;
  return b;
}

function f_se(a:int_ptr, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a[0];
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1056104, c + 8);
  g_a = c + 32;
  return b;
}

function f_te(a:int_ptr, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a[0];
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1067400, c + 8);
  g_a = c + 32;
  return b;
}

function f_ue(a:int, b:int) {
  var c:int = g_a - 48;
  g_a = c;
  if (eqz(0[1076488]:ubyte)) goto B_a;
  (c + 28)[0]:int = 1;
  c[3]:long@4 = 2L;
  c[2]:int = 1068300;
  c[9]:int = 29;
  c[11]:int = a;
  c[6]:int = c + 32;
  c[8]:int = c + 44;
  f_ag(c + 8, 1068340);
  unreachable;
  label B_a:
  g_a = c + 48;
}

function f_ve(a:int_ptr, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a[0];
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1072836, c + 8);
  g_a = c + 32;
  return b;
}

function f_we(a:int, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a;
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1049128, c + 8);
  g_a = c + 32;
  return b;
}

function f_xe(a:int, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a;
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1056104, c + 8);
  g_a = c + 32;
  return b;
}

function f_ye(a:int, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  c[1]:int = a;
  (c + 8 + 16)[0]:long = (b + 16)[0]:long@4;
  (c + 8 + 8)[0]:long = (b + 8)[0]:long@4;
  c[1]:long = b[0];
  b = f_va(c + 4, 1072836, c + 8);
  g_a = c + 32;
  return b;
}

function f_ze(a:{ a:long, b:long }, b:long, c:long, d:int) {
  var e:long;
  if (d & 64) goto B_b;
  if (eqz(d)) goto B_a;
  b = c << i64_extend_i32_u(0 - d & 63) |
      b >> (e = i64_extend_i32_u(d & 63));
  c = c >> e;
  goto B_a;
  label B_b:
  b = c >> i64_extend_i32_u(d & 63);
  c = 0L;
  label B_a:
  a.a = b;
  a.b = c;
}

function f_af(a:int, b:int_ptr, c:int, d:int, e:int) {
  var f:int = g_a - 16;
  g_a = f;
  if (eqz(c)) goto B_c;
  b = b[0];
  loop L_d {
    f_jc(f + 8, b, d, e);
    if (f[8]:ubyte != 4) goto B_b;
    c = c + -1;
    if (c) continue L_d;
  }
  label B_c:
  a[0]:byte = 4;
  goto B_a;
  label B_b:
  a[0]:long@4 = f[1]:long;
  label B_a:
  g_a = f + 16;
}

function f_bf(a:int, b:int):int {
  var c:int_ptr = g_a - 16;
  g_a = c;
  f_gg(c, b, 1069240, 13);
  c[3] = a;
  f_rb(c, 1069253, 5, c + 12, 1069260);
  c[3] = a + 12;
  f_rb(c, 1069276, 5, c + 12, 1069284);
  a = f_fe(c);
  g_a = c + 16;
  return a;
}

function f_cf(a:{ a:int, b:int }, b:int) {
  var c:int = 0;
  var d:int = 4;
  if (b < 5) goto B_a;
  d = b;
  br_table[B_a, B_b, ..B_c](b + -5)
  label B_c:
  b = b + -7;
  c = 1;
  d = 6;
  goto B_a;
  label B_b:
  b = 0;
  c = 1;
  d = 5;
  label B_a:
  a.b = c;
  a.a = d;
  (a + 8)[0]:int = b;
}

function f_df(a:int, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  f_qc(c + 8, a);
  a = c[3]:int;
  var d:int = c[2]:int;
  (c + 16 + 8)[0]:int = (b + 8)[0]:int;
  c[2]:long = b[0];
  b = f_tf(c + 16, d, a);
  g_a = c + 32;
  return b;
}

function f_ef(a:int, b:long_ptr@4):int {
  var c:int = g_a - 32;
  g_a = c;
  f_oc(c + 8, a);
  a = c[3]:int;
  var d:int = c[2]:int;
  (c + 16 + 8)[0]:int = (b + 8)[0]:int;
  c[2]:long = b[0];
  b = f_tf(c + 16, d, a);
  g_a = c + 32;
  return b;
}

function f_ff(a:int):int {
  var b:{ a:long, b:int } = g_a - 64;
  g_a = b;
  b.b = 0;
  b.a = 1L;
  f_fg(b + 16, b, 1054720);
  if (eqz(f_oe(a, b + 16))) goto B_a;
  f_ee(1054744, 55, b + 56, 1055036, 1054876);
  unreachable;
  label B_a:
  a = f_n(b);
  g_a = b + 64;
  return a;
}

function f_gf(a:int, b:int):int {
  var c:long_ptr = g_a - 16;
  g_a = c;
  if (eqz(a[3]:int)) goto B_b;
  b = a;
  goto B_a;
  label B_b:
  (c + 8)[0]:int = (a + 8)[0]:int;
  c[0] = a[0]:long@4;
  b = f_ef(b, c);
  f_mi(a, 20, 4);
  label B_a:
  g_a = c + 16;
  return b;
}

function f_hf(a:int_ptr, b:int, c:int):int {
  var e:int_ptr;
  var d:int_ptr;
  d = a[0];
  if ((d + 4)[0]:int - (a = (e = d + 8)[0]) >= c) goto B_a;
  f_hd(d, a, c);
  a = e[0];
  label B_a:
  f_dk(d[0] + a, b, c);
  e[0] = a + c;
  return 0;
}

function f_if(a:int_ptr, b:int, c:int):int {
  var e:int_ptr;
  var d:int_ptr;
  d = a[0];
  if ((d + 4)[0]:int - (a = (e = d + 8)[0]) >= c) goto B_a;
  f_jd(d, a, c);
  a = e[0];
  label B_a:
  f_dk(d[0] + a, b, c);
  e[0] = a + c;
  return 0;
}

function f_jf(a:int_ptr, b:int, c:int):int {
  var e:int_ptr;
  var d:int_ptr;
  d = a[0];
  if ((d + 4)[0]:int - (a = (e = d + 8)[0]) >= c) goto B_a;
  f_kd(d, a, c);
  a = e[0];
  label B_a:
  f_dk(d[0] + a, b, c);
  e[0] = a + c;
  return 0;
}

function f_kf(a:int_ptr, b:int, c:int):int {
  var d:int_ptr;
  var e:int;
  if ((a + 4)[0]:int - (e = (d = a + 8)[0]) >= c) goto B_a;
  f_hd(a, e, c);
  e = d[0];
  label B_a:
  f_dk(a[0] + e, b, c);
  d[0] = e + c;
  return 0;
}

function f_lf(a:int_ptr, b:int, c:int):int {
  var d:int_ptr;
  var e:int;
  if ((a + 4)[0]:int - (e = (d = a + 8)[0]) >= c) goto B_a;
  f_jd(a, e, c);
  e = d[0];
  label B_a:
  f_dk(a[0] + e, b, c);
  d[0] = e + c;
  return 0;
}

function f_mf(a:int_ptr, b:int) {
  var c:int = g_a - 32;
  g_a = c;
  if (b) goto B_a;
  a[1] = 0;
  g_a = c + 32;
  return ;
  label B_a:
  (c + 28)[0]:int = 0;
  c[6]:int = 1069020;
  c[3]:long@4 = 1L;
  c[2]:int = 1069048;
  f_ag(c + 8, 1069136);
  unreachable;
}

function f_nf(a:ubyte_ptr, b:ubyte_ptr, c:int):int {
  var f:int;
  var e:int;
  var d:int = 0;
  if (eqz(c)) goto B_a;
  loop L_c {
    e = a[0];
    if (e != (f = b[0])) goto B_b;
    a = a + 1;
    b = b + 1;
    c = c + -1;
    if (eqz(c)) goto B_a;
    continue L_c;
  }
  label B_b:
  d = e - f;
  label B_a:
  return d;
}

function f_of(a:int_ptr, b:int, c:int) {
  var d:int_ptr;
  var e:int;
  if ((a + 4)[0]:int - (e = (d = a + 8)[0]) >= c) goto B_a;
  f_jd(a, e, c);
  e = d[0];
  label B_a:
  f_dk(a[0] + e, b, c);
  d[0] = e + c;
}

function f_pf(a:int_ptr, b:int, c:int, d:int):int {
  var e:int;
  if (b == 1114112) goto B_c;
  e = 1;
  if (call_indirect(a[6], b, ((a + 28)[0]:int)[4]:int)) goto B_b;
  label B_c:
  if (c) goto B_a;
  e = 0;
  label B_b:
  return e;
  label B_a:
  return call_indirect(a[6], c, d, ((a + 28)[0]:int)[3]:int);
}

function f_qf(a:double):int {
  var c:long;
  var b:int;
  c = i64_reinterpret_f64(a);
  if (eqz(eqz(c & 9223372036854775807L))) goto B_a;
  return 2;
  label B_a:
  var d:long = c & 9218868437227405312L;
  if (d == 9218868437227405312L) goto B_c;
  b = 4;
  if (d != 0L) goto B_b;
  return 3;
  label B_c:
  b = eqz(c & 4503599627370495L);
  label B_b:
  return b;
}

function f_rf(a:int, b:int, c:int) {
  var d:int = g_a - 32;
  g_a = d;
  (d + 20)[0]:int = 0;
  d[4]:int = 1072168;
  d[1]:long@4 = 1L;
  d[7]:int = b;
  d[6]:int = a;
  d[0]:int = d + 24;
  f_ag(d, c);
  unreachable;
}

function f_sf(a:int, b:int) {
  var c:int = g_a - 32;
  g_a = c;
  (c + 20)[0]:int = 1;
  c[1]:long@4 = 1L;
  c[0]:int = 1072300;
  c[7]:int = 69;
  c[6]:int = a;
  c[4]:int = c + 24;
  f_ag(c, b);
  unreachable;
}

function f_tf(a:long_ptr@4, b:int, c:int):int {
  var d:int;
  d = f_wh(20, 4);
  if (d) goto B_a;
  f_mj(20, 4);
  unreachable;
  label B_a:
  d[4]:int = c;
  d[3]:int = b;
  d[0]:long@4 = a[0];
  (d + 8)[0]:int = (a + 8)[0]:int;
  return d;
}

function f_uf(a:{ a:int, b:int }, b:{ a:int, b:int }) {
  var c:int = b.b;
  var d:int = b.a;
  b = f_wh(8, 4);
  if (b) goto B_a;
  f_mj(8, 4);
  unreachable;
  label B_a:
  b.b = c;
  b.a = d;
  a.b = 1068432;
  a.a = b;
}

function f_vf(a:double_ptr, b:int_ptr):int {
  var c:int = b[0] & 1;
  var d:double = a[0];
  if (b[4] != 1) goto B_a;
  return f_da(b, d, c, (b + 20)[0]:int);
  label B_a:
  return f_la(b, d, c, 0);
}

function f_wf(a:long_ptr@4, b:int, c:int, d:int) {
  var e:{ a:int, b:int, c:byte }
  e = f_wh(12, 4);
  if (e) goto B_a;
  f_mj(12, 4);
  unreachable;
  label B_a:
  e.c = b;
  e.b = d;
  e.a = c;
  a[0] = i64_extend_i32_u(e) << 32L | 3L;
}

function f_xf(a:{ a:int, b:int, c:int }, b:int, c:int) {
  var d:int = memory_grow(c >> 16);
  a.c = 0;
  a.b = select_if(0, c & -65536, c = d == -1);
  a.a = select_if(0, d << 16, c);
}

function f_yf(a:int) {
  var b:{ a:int, b:int, c:int } = g_a - 16;
  g_a = b;
  var c:int = f_lh(f_sk(a), 1068384);
  var d:int = f_mh(f_rk(a));
  b.c = c;
  b.b = a;
  b.a = d;
  f_th(b);
  unreachable;
}

function f_zf() {
  var a:int = g_a - 32;
  g_a = a;
  (a + 28)[0]:int = 0;
  a[6]:int = 1069168;
  a[3]:long@4 = 1L;
  a[2]:int = 1069216;
  f_ag(a + 8, 1069224);
  unreachable;
}

function f_ag(a:int, b:int) {
  var c:int = g_a - 32;
  g_a = c;
  c[24]:byte = 1;
  c[5]:int = b;
  c[4]:int = a;
  c[3]:int = 1072284;
  c[2]:int = 1072168;
  f_yf(c + 8);
  unreachable;
}

export function alloc(a:int):int {
  var b:int;
  if (a < 0) goto B_c;
  if (a) goto B_b;
  b = 1;
  goto B_a;
  label B_c:
  f_zf();
  unreachable;
  label B_b:
  b = f_wh(a, 1);
  if (b) goto B_a;
  f_mj(a, 1);
  unreachable;
  label B_a:
  return b;
}

function f_cg(a:int):int {
  var b:long = a[0]:long@4;
  a = f_wh(20, 4);
  if (a) goto B_a;
  f_mj(20, 4);
  unreachable;
  label B_a:
  a[3]:long@4 = 0L;
  a[1]:long@4 = b;
  a[0]:int = 1;
  return a;
}

function f_dg(a:int_ptr, b:int):int {
  a = a[0];
  if (f_ki(b)) goto B_a;
  if (f_li(b)) goto B_b;
  return f_yi(a, b);
  label B_b:
  return f_vd(a, b);
  label B_a:
  return f_ud(a, b);
}

function f_eg(a:int, b:int, c:int, d:int, e:int) {
  var f:{ a:int, b:int, c:int, d:int } = g_a - 16;
  g_a = f;
  f.d = d;
  f.c = c;
  f.b = b;
  f.a = a;
  f_hh(f);
  unreachable;
}

function f_fg(a:int, b:int, c:int) {
  a[32]:byte = 3;
  a[0]:long@4 = 137438953472L;
  a[6]:int = b;
  a[4]:int = 0;
  a[2]:int = 0;
  (a + 28)[0]:int = c;
}

function f_gg(a:{ a:int, b:byte, c:byte }, b:int_ptr, c:int, d:int) {
  c = call_indirect(b[6], c, d, ((b + 28)[0]:int)[3]:int);
  a.c = 0;
  a.b = c;
  a.a = b;
}

function f_hg(a:{ a:int, b:byte, c:byte }, b:int_ptr) {
  var c:int = call_indirect(b[6], 1072615, 1, ((b + 28)[0]:int)[3]:int);
  a.c = 0;
  a.b = c;
  a.a = b;
}

function f_ig(a:int_ptr, b:int) {
  a[1] = ((a[1] & 1) | b) | 2;
  a = a + b;
  a[1] = a[1] | 1;
}

export function query(a:int, b:int, c:int, d:int):(int, int) {
  var e:{ a:int, b:int } = g_a - 16;
  g_a = e;
  f_pa(e, a, b, c, d);
  let t0, t1 = e.a, e.b;
  g_a = e + 16;
  return t0, t1;
}

function f_kg(a:int):int {
  var b:int = 1;
  if (a[4]:ubyte) goto B_a;
  a = a[0]:int;
  b = call_indirect(a[6]:int, 1072632, 1, (a[7]:int)[3]:int);
  label B_a:
  return b;
}

function f_lg(a:int_ptr, b:int):int {
  a = a[0];
  return 
    f_dc(
      select_if(i64_extend_i32_u(a), i64_extend_i32_s(a ^ -1) + 1L, a = a > -1),
      a,
      b);
}

function f_mg(a:int_ptr) {
  var b:int = (a + 4)[0]:int;
  if (eqz(b)) goto B_a;
  a = a[0];
  if (eqz(a)) goto B_a;
  f_mi(a, b, 1);
  label B_a:
}

function f_ng(a:int_ptr) {
  var b:int = a[1];
  if (eqz(b)) goto B_a;
  a = (a + 8)[0]:int;
  if (eqz(a)) goto B_a;
  f_mi(b, a, 1);
  label B_a:
}

function f_og(a:int, b:int) {
  var c:int_ptr = g_a - 16;
  g_a = c;
  c[3] = b;
  c[2] = a;
  f_zk(c + 8);
  unreachable;
}

function f_pg(a:{ a:int, b:int }, b:int):int {
  var c:int = 0;
  var d:int = a.a;
  if (d > b) goto B_a;
  c = d + a.b > b;
  label B_a:
  return c;
}

function f_qg(a:int, b:int, c:int) {
  var d:int_ptr = g_a - 16;
  g_a = d;
  d[3] = b;
  d[2] = a;
  f_sf(d + 8, c);
  unreachable;
}

export function init(a:int, b:int):(int, int) {
  var c:{ a:int, b:int } = g_a - 16;
  g_a = c;
  f_p(c, a, b);
  let t0, t1 = c.a, c.b;
  g_a = c + 16;
  return t0, t1;
}

function f_sg(a:int_ptr, b:int, c:int_ptr) {
  c[1] = c[1] & -2;
  a[1] = b | 1;
  (a + b)[0]:int = b;
}

function f_tg(a:long_ptr, b:int):int {
  var d:long;
  var c:long = a[0];
  return f_dc(c + (d = c >> 63L) ^ d, c > -1L, b);
}

function f_ug(a:int_ptr, b:int) {
  a[1] = b | 3;
  a = a + b;
  a[1] = a[1] | 1;
}

function f_vg(a:{ a:int, b:int }, b:int, c:int, d:int) {
  if (b) goto B_a;
  a.b = d;
  a.a = c;
  return ;
  label B_a:
  f_mj(c, d);
  unreachable;
}

function f_wg(a:int_ptr) {
  var b:int = (a + 4)[0]:int;
  if (eqz(b)) goto B_a;
  f_mi(a[0], b, 1);
  label B_a:
}

function f_xg(a:int_ptr) {
  var b:int = (a + 4)[0]:int;
  if (eqz(b)) goto B_a;
  f_mi(a[0], b, 1);
  label B_a:
}

function f_yg(a:int_ptr, b:int, c:int) {
  a[0] = 0;
  (a + 16)[0]:int = 0;
  (a + 8)[0]:long@4 = 0L;
}

function f_zg(a:int_ptr) {
  var b:int = (a + 4)[0]:int;
  if (eqz(b)) goto B_a;
  f_mi(a[0], b, 1);
  label B_a:
}

function f_ah(a:int_ptr) {
  var b:int = (a + 4)[0]:int;
  if (eqz(b)) goto B_a;
  f_mi(a[0], b, 1);
  label B_a:
}

function f_bh(a:ubyte_ptr, b:int):int {
  if (a[0]) goto B_a;
  return f_aa(b, 1072928, 5);
  label B_a:
  return f_aa(b, 1072924, 4);
}

function f_ch(a:int_ptr):int {
  var b:int;
  b = a[4];
  if (b) goto B_a;
  b = (a + 20)[0]:int;
  label B_a:
  return b;
}

function f_dh(a:{ a:int, b:int }, b:int_ptr) {
  a.b = (b + 8)[0]:int;
  a.a = b[0];
}

function f_eh(a:int, b:int) {
  var c:int;
  call_indirect(a, b, select_if(c = 0[269123]:int, 52, c));
  unreachable;
}

function f_fh(a:int):int {
  return select_if(0, 25 - (a >> 1), a == 31)
}

function f_gh(a:int_ptr, b:int) {
  a[1] = b | 1;
  (a + b)[0]:int = b;
}

function f_hh(a:{ a:int, b:int, c:int, d:int }) {
  f_hi(a.a, a.b, a.c, a.d);
  unreachable;
}

function f_ih(a:int_ptr, b:int, c:int):int {
  return call_indirect(a[6], b, c, ((a + 28)[0]:int)[3]:int)
}

function f_jh(a:int, b:int_ptr):int {
  return call_indirect(b[6], 1075602, 5, ((b + 28)[0]:int)[3]:int)
}

function f_kh(a:{ a:int, b:int, c:int }, b:int, c:int) {
  a.c = 0;
  a.b = c;
  a.a = b;
}

function f_lh(a:int, b:int):int {
  if (a) goto B_a;
  f_rf(1067424, 43, b);
  unreachable;
  label B_a:
  return a;
}

function f_mh(a:int):int {
  if (a) goto B_a;
  f_rf(1067424, 43, 1068400);
  unreachable;
  label B_a:
  return a;
}

function f_nh(a:int, b:int):int {
  return a + b + -1 & 0 - b
}

export function dealloc(a:int, b:int) {
  if (eqz(b)) goto B_a;
  f_mi(a, b, 1);
  label B_a:
}

function f_ph(a:int, b:int, c:int, d:int):int {
  var e:int = f_fa(a, b, c, d);
  return e;
}

function f_qh(a:int_ptr, b:int):int {
  return f_xj(a[0], (a + 8)[0]:int, b)
}

function f_rh(a:int_ptr, b:int):int {
  return f_xj(a[0], (a + 8)[0]:int, b)
}

function f_sh(a:int_ptr, b:int):int {
  return f_ba(a[0], (a + 8)[0]:int, b)
}

function f_th(a:{ a:int, b:int, c:int }) {
  f_yd(a.a, a.b, a.c);
  unreachable;
}

function f_uh(a:int):int {
  a = a << 1;
  return a | 0 - a;
}

function f_vh(a:{ a:int, b:int }, b:int):int {
  return call_indirect(a.a, b, a.b[3]:int)
}

function f_wh(a:int, b:int):int {
  var c:int = f_jk(a, b);
  return c;
}

function f_xh(a:{ a:int, b:int }, b:int):int {
  return f_ni(a.a, a.b, b)
}

function f_yh(a:int_ptr, b:int):int {
  return f_ba(a[0], a[2], b)
}

function f_zh(a:{ a:int, b:int }, b:int):int {
  return f_ba(a.a, a.b, b)
}

function f_ai(a:{ a:int, b:int }, b:int):int {
  return f_xj(a.a, a.b, b)
}

function f_bi(a:int_ptr, b:int):int {
  f_zb(a[0], b);
  return 0;
}

function f_ci(a:{ a:int, b:int }, b:int) {
  a.b = 1068432;
  a.a = b;
}

function f_di(a:{ a:long, b:long }) {
  a.b = 2L;
  a.a = 1L;
}

function f_ei(a:ubyte_ptr):int {
  return (a[4] & 2) >> 1
}

function f_fi(a:int_ptr) {
  a[1] = a[1] & -2
}

function f_gi(a:int_ptr):int {
  return (a[1] & 3) != 1
}

function f_hi(a:int, b:int, c:int, d:int) {
  f_s(a, b, c, d);
  unreachable;
}

function f_ii(a:{ a:int, b:int }, b:int):int {
  return f_aa(b, a.a, a.b)
}

function f_ji(a:ubyte_ptr):int {
  return (a[0] & 4) >> 2
}

function f_ki(a:ubyte_ptr):int {
  return (a[0] & 16) >> 4
}

function f_li(a:ubyte_ptr):int {
  return (a[0] & 32) >> 5
}

function f_mi(a:int, b:int, c:int) {
  f_ok(a, b, c)
}

function f_ni(a:int, b:int_ptr, c:int):int {
  return call_indirect(a, c, b[3])
}

function f_oi(a:int):int {
  return 0 - a & a
}

function f_pi(a:ubyte_ptr):int {
  return eqz(a[4] & 3)
}

function f_qi(a:int_ptr, b:int) {
  a[1] = b | 3
}

function f_ri(a:{ a:int, b:int }):int {
  return a.a + a.b
}

function f_si(a:int, b:int):int {
  return f_cd(a + 12, b)
}

function f_ti(a:int_ptr, b:int):int {
  a[0];
  return loop L_a {
           continue L_a
         }
}

function f_ui(a:uint_ptr, b:int):int {
  return f_dc(a[0], 1, b)
}

function f_vi(a:int, b:int, c:int) {
  f_qg(a, b, c);
  unreachable;
}

function f_wi(a:int, b:int, c:int):int {
  f_mc(a, b, c);
  return a;
}

function f_xi(a:int_ptr, b:int, c:int):int {
  return f_ia(a[0], b, c)
}

function f_yi(a:ubyte_ptr, b:int):int {
  return f_dc(a[0], 1, b)
}

function f_zi(a:long_ptr, b:int):int {
  return f_dc(a[0], 1, b)
}

function f_aj(a:int, b:int):int {
  return f_ih(b, 1049308, 5)
}

function f_bj(a:int, b:int) {
  f_nj(a, b)
}

function f_cj(a:int, b:int):int {
  return f_ih(b, 1067372, 8)
}

function f_dj(a:int, b:int):int {
  return f_ih(b, 1067380, 17)
}

function f_ej(a:int_ptr):int {
  return a[1] & -8
}

function f_fj(a:int_ptr):int {
  return a[1] & 1
}

function f_gj(a:int_ptr):int {
  return a[3] & 1
}

function f_hj(a:int_ptr):int {
  return a[3] >> 1
}

function f_ij(a:int_ptr, b:int):int {
  return f_dd(a[0], b)
}

function f_jj(a:int, b:int) {
  f_kj(a, b);
  unreachable;
}

function f_kj(a:int, b:int) {
  f_lj(a, b);
  unreachable;
}

function f_lj(a:int, b:int) {
  f_bj(a, b);
  unreachable;
}

function f_mj(a:int, b:int) {
  f_jj(a, b);
  unreachable;
}

function f_nj(a:int, b:int) {
  f_eh(a, b);
  unreachable;
}

function f_oj(a:int, b:int) {
  f_ke(a, b);
  unreachable;
}

function f_pj(a:int, b:int) {
  f_le(a, b);
  unreachable;
}

function f_qj(a:int, b:int) {
  f_me(a, b);
  unreachable;
}

function f_rj(a:int, b:int, c:int) {
  f_wj(a, b);
  unreachable;
}

function f_sj(a:int, b:int, c:int) {
  f_vj(a, b);
  unreachable;
}

function f_tj(a:int, b:int, c:int) {
  f_uj(a, b);
  unreachable;
}

function f_uj(a:int, b:int) {
  f_oj(a, b);
  unreachable;
}

function f_vj(a:int, b:int) {
  f_pj(a, b);
  unreachable;
}

function f_wj(a:int, b:int) {
  f_qj(a, b);
  unreachable;
}

function f_xj(a:int, b:int, c:int):int {
  return f_aa(c, a, b)
}

function f_yj(a:int_ptr, b:int):int {
  return f_fc(a[0], b)
}

function f_zj(a:int_ptr, b:int):int {
  return f_ec(a[0], b)
}

function f_ak(a:int_ptr, b:int):int {
  return f_cb(a[0], b)
}

function f_bk(a:int, b:int, c:int):int {
  return f_nd(a, b, c)
}

function f_ck(a:int, b:int, c:int):int {
  return f_nf(a, b, c)
}

function f_dk(a:int, b:int, c:int):int {
  return f_cc(a, b, c)
}

function f_ek(a:int, b:int, c:int):int {
  return f_ka(a, b, c)
}

function f_fk(a:int, b:int):int {
  return f_cj(a, b)
}

function f_gk(a:int, b:int):int {
  return f_dj(a, b)
}

function f_hk(a:int_ptr, b:int) {
  a[0] = 0
}

function f_ik(a:int_ptr, b:int) {
  a[0] = 0
}

function f_jk(a:int, b:int):int {
  return f_mb(a, b)
}

function f_kk(a:int, b:int):int {
  return a + b
}

function f_lk(a:int, b:int):int {
  return a - b
}

function f_mk(a:int):int {
  return a + 8
}

function f_nk(a:int):int {
  return a + -8
}

function f_ok(a:int, b:int, c:int) {
  f_x(a)
}

function f_pk(a:int_ptr):int {
  return a[3]
}

function f_qk(a:int_ptr):int {
  return a[2]
}

function f_rk(a:int_ptr):int {
  return a[2]
}

function f_sk(a:int_ptr):int {
  return a[3]
}

function f_tk(a:ubyte_ptr):int {
  return a[16]
}

function f_uk(a:int):long {
  return -3868628870407587612L
}

function f_vk(a:int):int {
  return 0
}

function f_wk(a:int):long {
  return -6278033084272672558L
}

function f_xk(a:int):long {
  return -5139102199292759541L
}

function f_yk(a:int):int {
  return 40
}

function f_zk(a:int):int {
  return unreachable
}

function f_al():int {
  return 7
}

function f_bl():int {
  return 8
}

function f_cl(a:int):int {
  return a
}

function f_dl(a:int, b:int, c:int, d:int, e:int):int {
  return 0
}

function f_el(a:int, b:int, c:int, d:int):int {
  return 0
}

function f_fl(a:int, b:int, c:int):int {
  return 0
}

function f_gl(a:int, b:int):int {
  return 0
}

function f_hl(a:int):int {
  return 65536
}

function f_il(a:int):long {
  return 7408497789316495214L
}

function f_jl(a:int) {
}

function f_kl(a:int) {
}

function f_ll(a:int) {
}

function f_ml(a:int) {
}

function f_nl(a:int) {
}

function f_ol(a:int) {
}

function f_pl(a:int) {
}

function f_ql(a:int) {
}

function f_rl(a:int) {
}

