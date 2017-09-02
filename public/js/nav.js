
var menu = document.getElementById('menu');
menu.addEventListener('click', function() {
    var manv = document.getElementById('mnav');
        if (manv.style.height == 'auto') {
            manv.style.height = '0';
        }else{
            manv.style.height = 'auto';
        }

        // 3中方式相同
        // manv.style.height = 'auto';
        // manv.setAttribute('style', 'height: auto !important');
        // manv.style.setProperty( 'height',' auto', 'important');
}, false);


var submenu = document.getElementsByClassName('subs');

for (var i = 0; i < submenu.length; i++) {

    submenu[i].addEventListener('click', function() {

            var subitem = this.nextElementSibling;
            if (subitem.style.height == 'auto') {
                subitem.style.height = '0';
            }else{
                subitem.style.height = 'auto';
            }

            // 3中方式相同
            // manv.style.height = 'auto';
            // manv.setAttribute('style', 'height: auto !important');
            // manv.style.setProperty( 'height',' auto', 'important');
    }, false);
        
}