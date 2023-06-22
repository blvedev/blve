
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let count = reactiveValue(0, 1, refs)
    function increment(){
      count.v++
      console.log(count.v)
    }
    function clear(){
      if(interval.v){
        clearInterval(interval.v)
        interval.v = null
      }else{
        interval.v = setInterval(increment, 2000)
      }
    }
    let interval = reactiveValue(setInterval(increment, 2000), 2, refs)

    elm.innerHTML = `<div><h1 id="abc">Hello Blve!</h1><div id="test">${escapeHtml(count.v)}</div><button id="test">+1</button><button id="test">${escapeHtml(interval.v == null ? "start" : "clear")}</button></div>`;

    const [testRef,testRef,testRef] = getElmRefs(["test","test","test"], 7);

    addEvListener(testRef, "click", increment);

    addEvListener(testRef, "click", clear);

    refs[2] = genUpdateFunc(() => {
        refs[0] & 1 && replaceText(`${escapeHtml(count.v)}`, testRef);
    
        refs[0] & 2 && replaceText(`${escapeHtml(interval.v == null ? "start" : "clear")}`, testRef);
    
    });

}