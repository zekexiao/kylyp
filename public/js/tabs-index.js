onload=function(){
    var links = document.getElementById("top").getElementsByTagName('li')
    links_len=links.length;
    for(var i=0; i<links_len; i++){
        links[i].onmouseover=function(){
            clearInterval(iIntervalId);
            this.onmouseout=function(){
                iIntervalId = setInterval(setTab('one',1),ScrollTime);;
            }
        }
    }
}
function setTab(name,cursel){
    cursel_0=cursel;
    for(var i=1; i<=links_len; i++){
        var menu = document.getElementById(name+i);
        var menudiv = document.getElementById("con_"+name+"_"+i);
        if(i==cursel){
            menu.className="off";
            menudiv.style.display="block";
        }
        else{
            menu.className="";
            menudiv.style.display="none";
        }
    }
}
