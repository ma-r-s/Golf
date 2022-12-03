FROM nvidia/cuda:11.7.1-cudnn8-devel-ubuntu20.04
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]
#Install ROS Noetic
RUN apt update
RUN apt install curl -y
RUN sh -c 'echo "deb http://packages.ros.org/ros/ubuntu focal main" > /etc/apt/sources.list.d/ros-latest.list'
RUN curl -s https://raw.githubusercontent.com/ros/rosdistro/master/ros.asc | apt-key add -
RUN apt update
RUN apt install ros-noetic-ros-base -y
RUN echo "source /opt/ros/noetic/setup.bash" >> ~/.bashrc
#Install build tools
RUN apt install python3-rosdep python3-rosinstall python3-rosinstall-generator python3-wstool build-essential -y
RUN rosdep init && rosdep update
#Install dependencies
RUN apt install ros-noetic-velodyne -y
RUN apt upgrade -y
COPY . .
RUN apt install ros-noetic-rosbridge-suite -y
EXPOSE 9090