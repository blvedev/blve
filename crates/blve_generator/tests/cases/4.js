
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    const inputValue = reactiveValue("", 1, refs)
    function setFoo() {
      inputValue.v = "foo"
    }

    elm.innerHTML = `<div><input id="hyDvUFpBXTawervJGaTte" /><div id="Lnd_NTiIdUuISofwdrcfg">inputValue: ${escapeHtml(inputValue.v)}</div><button id="RmLUCoG$cpyVbsOddOylu">set foo</button></div>`;

    const [hyDvUFpBXTawervJGaTteRef,Lnd_NTiIdUuISofwdrcfgRef,RmLUCoG$cpyVbsOddOyluRef] = getElmRefs(["hyDvUFpBXTawervJGaTte","Lnd_NTiIdUuISofwdrcfg","RmLUCoG$cpyVbsOddOylu"], 7);

    addEvListener(hyDvUFpBXTawervJGaTteRef, "input", (e)=>inputValue.v = event.target.value);

    addEvListener(RmLUCoG$cpyVbsOddOyluRef, "click", setFoo);

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("value", inputValue.v, hyDvUFpBXTawervJGaTteRef);
    
        refs[0] & 1 && replaceText(`inputValue: ${escapeHtml(inputValue.v)}`, Lnd_NTiIdUuISofwdrcfgRef);
    
    });

}