### How to use Development environment

1. Go To Action tabs of The page
2. Select Any runner you want to run
![Screenshot 2024-10-05 at 12 20 40 AM](https://github.com/user-attachments/assets/8106b2e8-c30b-4a04-ab54-8cd911abb294)
3. Run the selected Workflow with tmate debug mode enable to get ssh for that runner
![Screenshot 2024-10-05 at 12 22 37 AM](https://github.com/user-attachments/assets/5e0d28af-1649-471b-b9b0-478ef094fafa)
4. Refresh the page so The cureent Running workflow will be visible
![Screenshot 2024-10-05 at 12 26 25 AM](https://github.com/user-attachments/assets/6d26cca2-75ab-4b00-95e6-c391503dffa8)
5. select that workflow
6. Observe the build process, Wait till we go to tmate action
7. In action look for ssh key 
![Screenshot 2024-10-05 at 12 23 45 AM](https://github.com/user-attachments/assets/e2addb82-612a-4158-8370-f85ae195aa92)
8. Copy that ssh key and pest it on your terminal
![Screenshot 2024-10-05 at 12 24 06 AM](https://github.com/user-attachments/assets/bbff1a6b-37b8-4e2a-900c-70327e9f6d6c)
9. When popup open from tmate press ctrl + C to get actual terminal of system
![Screenshot 2024-10-05 at 12 24 20 AM](https://github.com/user-attachments/assets/8c7489eb-b43e-47a7-a653-c52e2fa11161)
10. Now you can use Host system
![Screenshot 2024-10-05 at 12 25 20 AM](https://github.com/user-attachments/assets/fe200c1e-49ff-44d8-9b1c-dcd3843640c7)
11. mfg-tool is there in root dir itself and system has already RUST 1.74 installed
![Screenshot 2024-10-05 at 12 25 35 AM](https://github.com/user-attachments/assets/6bc362a0-ffe6-4c45-9950-d5edcc45bac8)
