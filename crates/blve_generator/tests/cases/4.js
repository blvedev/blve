
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    const inputValue = reactiveValue("", 1, refs)
    function setFoo() {
      inputValue.v = "foo"
    }

    elm.innerHTML = `<div><input id="test" /><div id="test">inputValue: ${escapeHtml(inputValue.v)}</div><button id="test">set foo</button></div>`;

    const [testRef,testRef,testRef] = getElmRefs(["test","test","test"], 7);

    addEvListener(testRef, "input", (e)=>inputValue.v = event.target.value);

    addEvListener(testRef, "click", setFoo);

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("value", inputValue.v, testRef);
    
        refs[0] & 1 && replaceText(`inputValue: ${escapeHtml(inputValue.v)}`, testRef);
    
    });

}