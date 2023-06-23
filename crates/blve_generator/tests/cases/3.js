
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    const balue = reactiveValue("", 1, refs)
    function onInput(e) {
      balue.v = e.target.value
    }
    function xxx() {
      balue.v += "xxx"
    }

    elm.innerHTML = `<div><input id="mSZVkKzedMphuVkGNIHeF" value="${balue.v}" /><div id="tzPQlyiTuzVvo_BLFwCqV">${escapeHtml(balue.v)}</div><button id="syHUaALEsaXMHcAsLMvyE">xxx</button></div>`;

    const [mSZVkKzedMphuVkGNIHeFRef,tzPQlyiTuzVvo_BLFwCqVRef,syHUaALEsaXMHcAsLMvyERef] = getElmRefs(["mSZVkKzedMphuVkGNIHeF","tzPQlyiTuzVvo_BLFwCqV","syHUaALEsaXMHcAsLMvyE"], 7);

    addEvListener(mSZVkKzedMphuVkGNIHeFRef, "input", onInput);

    addEvListener(syHUaALEsaXMHcAsLMvyERef, "click", xxx);

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("value", balue.v, mSZVkKzedMphuVkGNIHeFRef);
    
        refs[0] & 1 && replaceText(`${escapeHtml(balue.v)}`, tzPQlyiTuzVvo_BLFwCqVRef);
    
    });

}