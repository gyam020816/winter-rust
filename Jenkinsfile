properties([[$class: 'GitLabConnectionProperty', gitLabConnection: 'gitlab']])
pipeline {
    agent {
        label 'docker'
    }
    options {
        skipDefaultCheckout true
    }
    stages {
        stage('Reset workspace') {
            steps {
                deleteDir()
                checkout scm
            }
        }
        stage('Build') {
            agent {
                dockerfile {
                    reuseNode true
                }
            }
            steps {
                sh "cargo build"
            }
        }
        stage('Test') {
            agent {
                dockerfile {
                    reuseNode true
                }
            }
            steps {
                sh "cargo test --verbose"
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
