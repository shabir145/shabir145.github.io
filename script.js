document.addEventListener("DOMContentLoaded", () => {
  const linuxLink = document.getElementById("download-linux");
  const windowsLink = document.getElementById("download-windows");

  linuxLink.addEventListener("click", () => {
    alert("Thank you for downloading the Linux version!");
  });

  windowsLink.addEventListener("click", () => {
    alert("Thank you for downloading the Windows version!");
  });
});

