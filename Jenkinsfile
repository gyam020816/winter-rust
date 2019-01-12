properties([[$class: 'GitLabConnectionProperty', gitLabConnection: 'gitlab']])
pipeline {
    agent {
        label 'docker'
    }
    options {
        skipDefaultCheckout true
    }
    stages {
        stage('Clean workspace') {
            steps {
                deleteDir()
            }
        }
        stage('Build') {
            agent {
                docker {
                    image 'ekidd/rust-musl-builder'
                    reuseNode true
                }
            }
            steps {
                checkout scm
                sh "cargo build"
            }
        }
        stage('Test') {
            agent {
                docker {
                    image 'ekidd/rust-musl-builder'
                    reuseNode true
                }
            }
            steps {
                sh "cargo test"
            }
        }
    }
    post {
        always {
            step([$class: 'Mailer', notifyEveryUnstableBuild: true, recipients: 'automation@ha3.eu'])
        }
        success {
            updateGitlabCommitStatus name: 'jenkins', state: 'success'
        }
        failure {
            updateGitlabCommitStatus name: 'jenkins', state: 'failed'
        }
        unstable {
            updateGitlabCommitStatus name: 'jenkins', state: 'failed'
        }
    }
}
