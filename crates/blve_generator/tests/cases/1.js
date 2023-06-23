
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
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

    elm.innerHTML = `<div><h1 id="abc">Hello Blve!</h1><div id="EU$ACyJdPgqXwgTtZFQdm">${escapeHtml(count.v)}</div><button id="A_XVMHZjeJfEwLDOaYHGa">+1</button><button id="bmDvolUlQvqeLETyLNwHt">${escapeHtml(interval.v == null ? "start" : "clear")}</button></div>`;

    const [EU$ACyJdPgqXwgTtZFQdmRef,A_XVMHZjeJfEwLDOaYHGaRef,bmDvolUlQvqeLETyLNwHtRef] = getElmRefs(["EU$ACyJdPgqXwgTtZFQdm","A_XVMHZjeJfEwLDOaYHGa","bmDvolUlQvqeLETyLNwHt"], 7);

    addEvListener(A_XVMHZjeJfEwLDOaYHGaRef, "click", increment);

    addEvListener(bmDvolUlQvqeLETyLNwHtRef, "click", clear);

    refs[2] = genUpdateFunc(() => {
        refs[0] & 1 && replaceText(`${escapeHtml(count.v)}`, EU$ACyJdPgqXwgTtZFQdmRef);
    
        refs[0] & 2 && replaceText(`${escapeHtml(interval.v == null ? "start" : "clear")}`, bmDvolUlQvqeLETyLNwHtRef);
    
    });

}