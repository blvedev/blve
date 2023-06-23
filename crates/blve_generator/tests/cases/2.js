
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let color = reactiveValue('red', 1, refs)
    function yellow(){
      color.v = 'yellow'
    }
    function red(){
      color.v = 'red'
    }
    function blve(){
      color.v = 'blue'
    }

    elm.innerHTML = `<div><span id="hNXqc_j$VSr_JHwwmnANe" style="${`color : ${ color.v } `}">I am a color</span><button id="yKr_KeWCcdUTyYyxpyHCH">黄色</button><button id="lkIfmqlfoYznPmPwVKiqb">赤色</button><button id="LwywxHlYakgWiuXzdyqGW">青色</button></div>`;

    const [hNXqc_j$VSr_JHwwmnANeRef,yKr_KeWCcdUTyYyxpyHCHRef,lkIfmqlfoYznPmPwVKiqbRef,LwywxHlYakgWiuXzdyqGWRef] = getElmRefs(["hNXqc_j$VSr_JHwwmnANe","yKr_KeWCcdUTyYyxpyHCH","lkIfmqlfoYznPmPwVKiqb","LwywxHlYakgWiuXzdyqGW"], 15);

    addEvListener(yKr_KeWCcdUTyYyxpyHCHRef, "click", yellow);

    addEvListener(lkIfmqlfoYznPmPwVKiqbRef, "click", red);

    addEvListener(LwywxHlYakgWiuXzdyqGWRef, "click", blve);

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("style", `color : ${ color.v } `, hNXqc_j$VSr_JHwwmnANeRef);
    
    });

}