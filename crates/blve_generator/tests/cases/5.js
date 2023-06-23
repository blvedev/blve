
import { reactiveValue,getElmRefs,addEvListener,genUpdateFunc,escapeHtml,replaceText,replaceAttr,insertEmpty } from 'blve/dist/runtime'
export default function(elm) {
    const refs = [0, false, null];
    let inputValue = reactiveValue("", 1, refs)
    function addFoo() {
      inputValue.v += "foo"
    }

    elm.innerHTML = `<div><input id="eSpbDaslllcKBSCPpkgxM" /><div id="rGalXBsYRiNTicjdyIpJU">inputValue: ${escapeHtml(inputValue.v)}</div><button id="PboOoVHwCYqav$lckcooZ">setFoo</button><button id="ZcXiiemMeYLClwsdk_zbO">addFoo1</button><button id="wOdwZyaEOeaKSydaikEdt">addFoo2</button></div>`;

    const [eSpbDaslllcKBSCPpkgxMRef,rGalXBsYRiNTicjdyIpJURef,PboOoVHwCYqav$lckcooZRef,ZcXiiemMeYLClwsdk_zbORef,wOdwZyaEOeaKSydaikEdtRef] = getElmRefs(["eSpbDaslllcKBSCPpkgxM","rGalXBsYRiNTicjdyIpJU","PboOoVHwCYqav$lckcooZ","ZcXiiemMeYLClwsdk_zbO","wOdwZyaEOeaKSydaikEdt"], 31);

    addEvListener(eSpbDaslllcKBSCPpkgxMRef, "input", (e)=>inputValue.v = event.target.value);

    addEvListener(PboOoVHwCYqav$lckcooZRef, "click", ()=>inputValue.v = 'foo');

    addEvListener(ZcXiiemMeYLClwsdk_zbORef, "click", addFoo);

    addEvListener(wOdwZyaEOeaKSydaikEdtRef, "click", ()=>addFoo());

    refs[2] = genUpdateFunc(() => {
        refs[0]  & 1 && replaceAttr("value", inputValue.v, eSpbDaslllcKBSCPpkgxMRef);
    
        refs[0] & 1 && replaceText(`inputValue: ${escapeHtml(inputValue.v)}`, rGalXBsYRiNTicjdyIpJURef);
    
    });

}