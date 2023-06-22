
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    const balue = reactiveValue("", 1, refs)
    function onInput(e) {
      balue.v = e.target.value
    }
    function xxx() {
      balue.v += "xxx"
    }

    elm.innerHTML = `<div><input id="test" value="${balue.v}" /><div id="test">${escapeHtml(balue.v)}</div><button id="test">xxx</button></div>`;

    const [testRef,testRef,testRef] = getElmRefs(["test","test","test"], 7);

    addEvListener(testRef, "input", onInput);

    addEvListener(testRef, "click", xxx);

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("value", balue.v, testRef);
    
        refs[0] & 1 && replaceText(`${escapeHtml(balue.v)}`, testRef);
    
    });

}