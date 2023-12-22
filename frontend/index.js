/* Toggle between adding and removing the "responsive" class to topnav when the user clicks on the icon */
function toggle_menu() {
    var menu_button = document.getElementById("menu_button");
    menu_button.classList.toggle("change");

    var x = document.getElementById("topnav");
    if (x.className === "topnav") {
      x.className += " responsive";
    } else {
      x.className = "topnav";
    }
}
