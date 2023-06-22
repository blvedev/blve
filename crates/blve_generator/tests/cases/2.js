
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr } from 'blve/dist/runtime'
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

    elm.innerHTML = `<div><span id="test" style="${`color : ${ color.v } `}">I am a color</span><button id="test">黄色</button><button id="test">赤色</button><button id="test">青色</button></div>`;

    const [testRef,testRef,testRef,testRef] = getElmRefs(["test","test","test","test"], 15);

    addEvListener(testRef, "click", yellow);

    addEvListener(testRef, "click", red);

    addEvListener(testRef, "click", blve);

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("style", `color : ${ color.v } `, testRef);
    
    });

}