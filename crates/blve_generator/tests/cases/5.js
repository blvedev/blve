
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let inputValue = reactiveValue("", 1, refs)
    function addFoo() {
      inputValue.v += "foo"
    }

    elm.innerHTML = `<div><input id="test" /><div id="test">inputValue: ${escapeHtml(inputValue.v)}</div><button id="test">setFoo</button><button id="test">addFoo1</button><button id="test">addFoo2</button></div>`;

    const [testRef,testRef,testRef,testRef,testRef] = getElmRefs(["test","test","test","test","test"], 31);

    addEvListener(testRef, "input", (e)=>inputValue.v = event.target.value);

    addEvListener(testRef, "click", ()=>inputValue.v = 'foo');

    addEvListener(testRef, "click", addFoo);

    addEvListener(testRef, "click", ()=>addFoo());

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("value", inputValue.v, testRef);
    
        refs[0] & 1 && replaceText(`inputValue: ${escapeHtml(inputValue.v)}`, testRef);
    
    });

}