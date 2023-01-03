cmake_minimum_required(VERSION 3.14)

project(fabric-tools LANGUAGES)

add_custom_target(docker_build
  COMMAND docker build -f docker/Dockerfile -t service-fabric-onebox:v1 .
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

# this starts a debug container in a new window.
# SF has lease driver that needs hyper-v isolation
add_custom_target(docker_run
  COMMAND start powershell -noexit -command "docker run --name sf-test --isolation=hyperv -it --rm service-fabric-onebox:v1"
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)